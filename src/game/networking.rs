use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::runtime::*;

use tokio::sync::mpsc;

use crate::game::game::Move;

const MSG_MAGIC_NUM: [u8; 2] = [0x3a, 0x4b];
pub enum Message {
    // Requests server to start a match against another player
    // Server will match players with the same password
    RequestMatch(String),
    // Server tells client that game has started, and sends what colour client is
    StartGame(i32),
    // Send/Recieve a Chess move
    Move(Move),
    // Request server to end matchmaking/match (resign) OR server requests end match
    EndMatch,
    // Client has been disconnected
    Disconnected,
    // Client has been connected
    Connected,
    // Error has occured
    Error(String),
}

pub struct ChessClient {
    _addr: String,
    _rt: Runtime,
    tx_send: mpsc::UnboundedSender<Message>,
    rx_recv: mpsc::UnboundedReceiver<Message>,

}

impl ChessClient {
    pub fn new(addr: String) -> Result<ChessClient, String> {
        let (tx_send, tx_recv) = mpsc::unbounded_channel::<Message>();
        let (rx_send, rx_recv) = mpsc::unbounded_channel::<Message>();

        let rt = Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .map_err(|_e| "Failed to create tokio runtime".to_string())?;

        rt.spawn(ChessClient::connect(addr.clone(), rx_send, tx_recv));


        let client = ChessClient {
            _addr: addr,
            _rt: rt,

            tx_send: tx_send,
            rx_recv: rx_recv,
        };

        Ok(client)
    }

    async fn connect(addr: String, rx_send: mpsc::UnboundedSender<Message>, tx_recv: mpsc::UnboundedReceiver<Message>) {
        let stream = match TcpStream::connect(&addr).await {
            Ok(s) => s,
            Err(e) => {
                let _ = rx_send.send(Message::Error(e.to_string()));
                let _ = rx_send.send(Message::Disconnected);
                return;
            }
        };

        let _ = rx_send.send(Message::Connected);
        let (reader, writer) = stream.into_split();

        tokio::select! {
            _ = ChessClient::tx_worker(writer, rx_send.clone(), tx_recv) => {

            }
            _ = ChessClient::rx_worker(reader, rx_send) => {

            }
        };
    }

    async fn tx_worker(mut writer: tokio::net::tcp::OwnedWriteHalf, rx_send: mpsc::UnboundedSender<Message>, mut tx_recv: mpsc::UnboundedReceiver<Message>) {
        let mut msg_buf: Vec<u8> = vec![];
        while let Some(msg) = tx_recv.recv().await {
            msg_buf.clear();
            msg_buf.push(MSG_MAGIC_NUM[0]);
            msg_buf.push(MSG_MAGIC_NUM[1]);
            match msg {
                Message::RequestMatch(str) => {
                    let mut bytes_vec = str.into_bytes();
                    let mut len = (bytes_vec.len() as u32).to_be_bytes().to_vec();
                    msg_buf.push('r' as u8);
                    msg_buf.append(&mut len);
                    msg_buf.append(&mut bytes_vec);
                },
                Message::Move(mov) => {
                    let mut org = mov.org_square.to_be_bytes().to_vec();
                    let mut tgt = mov.tgt_square.to_be_bytes().to_vec();
                    msg_buf.push('m' as u8);
                    msg_buf.append(&mut org);
                    msg_buf.append(&mut tgt);
                },
                Message::EndMatch => {
                    msg_buf.push('x' as u8);
                }
                _ => {
                    let _ = rx_send.send(Message::Error("Message Type Invalid".to_string()));
                }
            }

            if writer.write_all(&msg_buf.as_slice()).await.is_err() {
                let _ = rx_send.send(Message::Error("Failed to send message".to_string()));
                break;
            }
        }

        let _ = rx_send.send(Message::Disconnected);
    }

    async fn rx_worker(mut reader: tokio::net::tcp::OwnedReadHalf, rx_send: mpsc::UnboundedSender<Message>) {
        loop {
            let mut header: [u8; 3] = [0,0,0];
            if reader.read_exact(&mut header).await.is_err() {
                let _ = rx_send.send(Message::Error("Failed to read header".to_string()));
                break;
            }

            if header[0] != MSG_MAGIC_NUM[0] || header[1] != MSG_MAGIC_NUM[1] {
                continue;
            }

            let cmd: char = header[2] as char;
            match cmd {
                's' => { // Start match
                    let mut turn_buf: [u8; 4] = [0; 4];
                    if reader.read_exact(&mut turn_buf).await.is_err() {
                        let _ = rx_send.send(Message::Error("Failed to read body".to_string()));
                        break;
                    }
                    let _ = rx_send.send(Message::StartGame(i32::from_be_bytes(turn_buf)));
                },
                'm' => { // Move
                    let mut rd_buf: [u8; 16] = [0; 16];
                    if reader.read_exact(&mut rd_buf).await.is_err() {
                        let _ = rx_send.send(Message::Error("Failed to read body".to_string()));
                        break;
                    }

                    let (org_slice, tgt_slice) = rd_buf.split_at(size_of::<usize>());
                    
                    let _ = rx_send.send(Message::Move(Move {
                        org_square: usize::from_be_bytes(org_slice.try_into().unwrap()),
                        tgt_square: usize::from_be_bytes(tgt_slice.try_into().unwrap()),
                    }));
                },
                'x' => { // End match
                    let _ = rx_send.send(Message::EndMatch);
                },
                _ => {
                    let _ = rx_send.send(Message::Error("Recieved Message Type Invalid".to_string()));
                }
            }
        }

        let _ = rx_send.send(Message::Disconnected);
    }

    pub fn request_match(&self, pass: &str) {
        let _ = self.tx_send.send(Message::RequestMatch(pass.to_string()));
    }

    pub fn send_move(&self, mov: Move) {
        let _ = self.tx_send.send(Message::Move(mov));
    }

    pub fn end_match(&self) {
        let _ = self.tx_send.send(Message::EndMatch);
    }

    pub fn poll_events(&mut self) -> Option<Message> {
        self.rx_recv.try_recv().ok()
    }


}