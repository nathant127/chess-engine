// server.c - minimal blocking relay server for the chess client
#include <arpa/inet.h>
#include <netinet/in.h>
#include <poll.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

#define MAGIC0 0x3a
#define MAGIC1 0x4b
#define MOVE_BODY_LEN 16 /* two usize values, 8 bytes each (64-bit client) */

/* ---------- I/O helpers ---------- */

// Read exactly n bytes. Returns 0 on success, -1 on EOF/error.
static int read_full(int fd, void *buf, size_t n) {
    uint8_t *p = buf;
    while (n > 0) {
        ssize_t r = recv(fd, p, n, 0);
        if (r <= 0) return -1;
        p += r;
        n -= (size_t)r;
    }
    return 0;
}

// Write exactly n bytes. Returns 0 on success, -1 on error.
static int write_full(int fd, const void *buf, size_t n) {
    const uint8_t *p = buf;
    while (n > 0) {
        ssize_t r = send(fd, p, n, MSG_NOSIGNAL);
        if (r <= 0) return -1;
        p += r;
        n -= (size_t)r;
    }
    return 0;
}

// Discard n bytes from the socket.
static int skip_bytes(int fd, size_t n) {
    uint8_t tmp[256];
    while (n > 0) {
        size_t chunk = n < sizeof tmp ? n : sizeof tmp;
        if (read_full(fd, tmp, chunk) < 0) return -1;
        n -= chunk;
    }
    return 0;
}

/* ---------- Protocol helpers ---------- */

static int send_start(int fd, int32_t turn) {
    uint8_t msg[7] = {MAGIC0, MAGIC1, 's'};
    uint32_t be = htonl((uint32_t)turn);
    memcpy(msg + 3, &be, 4);
    return write_full(fd, msg, sizeof msg);
}

static int send_end(int fd) {
    uint8_t msg[3] = {MAGIC0, MAGIC1, 'x'};
    return write_full(fd, msg, sizeof msg);
}

// Wait for a match request ('r' + u32 length + password) and ignore the password.
static int wait_for_request(int fd) {
    for (;;) {
        uint8_t hdr[3];
        if (read_full(fd, hdr, 3) < 0) return -1;
        if (hdr[0] != MAGIC0 || hdr[1] != MAGIC1) return -1;

        if (hdr[2] == 'r') {
            uint32_t len_be;
            if (read_full(fd, &len_be, 4) < 0) return -1;
            uint32_t len = ntohl(len_be);
            if (len > (1u << 20)) return -1; // sanity limit
            if (skip_bytes(fd, len) < 0) return -1;
            return 0;
        }
        if (hdr[2] == 'x') return -1; // cancelled matchmaking
        return -1;                    // anything else is unexpected here
    }
}

/* ---------- Match relay ---------- */

// Read one message from `from` and forward it to `to`.
// Returns 0 to keep going, -1 if the match should end.
static int relay_one(int from, int to) {
    uint8_t hdr[3];
    if (read_full(from, hdr, 3) < 0) {
        printf("  a player disconnected\n");
        return -1;
    }
    if (hdr[0] != MAGIC0 || hdr[1] != MAGIC1) {
        printf("  bad magic, ending match\n");
        return -1;
    }

    switch (hdr[2]) {
    case 'm': {
        uint8_t body[MOVE_BODY_LEN];
        if (read_full(from, body, sizeof body) < 0) return -1;
        uint8_t out[3 + MOVE_BODY_LEN];
        memcpy(out, hdr, 3);
        memcpy(out + 3, body, sizeof body);
        if (write_full(to, out, sizeof out) < 0) return -1;
        printf("  relayed a move\n");
        return 0;
    }
    case 'x':
        printf("  player ended the match\n");
        return -1;
    default:
        printf("  unknown command '%c', ignoring\n", hdr[2]);
        return 0;
    }
}

static void run_match(int a, int b) {
    struct pollfd pfds[2] = {{a, POLLIN, 0}, {b, POLLIN, 0}};

    for (;;) {
        if (poll(pfds, 2, -1) < 0) return;

        for (int i = 0; i < 2; i++) {
            if (pfds[i].revents & (POLLIN | POLLHUP | POLLERR)) {
                int from = pfds[i].fd;
                int to = pfds[1 - i].fd;
                if (relay_one(from, to) < 0) {
                    // Tell the other player the match is over (ignore failure,
                    // they may already be gone).
                    send_end(to);
                    return;
                }
            }
        }
    }
}

/* ---------- Main ---------- */

int main(int argc, char **argv) {
    int port = argc > 1 ? atoi(argv[1]) : 9000;
    signal(SIGPIPE, SIG_IGN);
    srand((unsigned)time(NULL));

    int lfd = socket(AF_INET, SOCK_STREAM, 0);
    if (lfd < 0) { perror("socket"); return 1; }

    int yes = 1;
    setsockopt(lfd, SOL_SOCKET, SO_REUSEADDR, &yes, sizeof yes);

    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_ANY);
    addr.sin_port = htons((uint16_t)port);

    if (bind(lfd, (struct sockaddr *)&addr, sizeof addr) < 0) { perror("bind"); return 1; }
    if (listen(lfd, 4) < 0) { perror("listen"); return 1; }
    printf("Listening on port %d\n", port);

    for (;;) {
        int players[2];

        // Accept two players and wait for each one's match request.
        for (int i = 0; i < 2; i++) {
            for (;;) {
                players[i] = accept(lfd, NULL, NULL);
                if (players[i] < 0) { perror("accept"); continue; }
                printf("Player %d connected, waiting for match request\n", i + 1);

                if (wait_for_request(players[i]) == 0) {
                    printf("Player %d requested a match\n", i + 1);
                    break;
                }
                printf("Player %d failed matchmaking, dropping\n", i + 1);
                close(players[i]);
            }
        }

        // Randomly pick who is white (0) and black (1).
        int white = rand() & 1;
        int black = 1 - white;
        printf("Starting match (player %d is white)\n", white + 1);

        if (send_start(players[white], 0) == 0 && send_start(players[black], 1) == 0) {
            run_match(players[white], players[black]);
        }

        printf("Match over\n");
        close(players[0]);
        close(players[1]);
    }
}