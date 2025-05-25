SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR

cargo build --release --target "x86_64-pc-windows-gnu"
cargo build --release 

RELEASE_FOLDER=chess_engine_release/

rm -rf $RELEASE_FOLDER
mkdir $RELEASE_FOLDER
cp -r assets/ $RELEASE_FOLDER
cp target/release/chess-engine $RELEASE_FOLDER
cp target/x86_64-pc-windows-gnu/release/chess-engine.exe $RELEASE_FOLDER

zip -r chess-engine-release.zip $RELEASE_FOLDER