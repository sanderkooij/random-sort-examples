echo testing native
cargo build --release
./target/release/ttt

echo testing wasm
cargo wasi build --release
wasmer ./target/wasm32-wasi/release/ttt.rustc.wasm 
