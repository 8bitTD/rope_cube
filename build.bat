rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name rope_cube --out-dir wasm/target --target web ./target/wasm32-unknown-unknown/release/rope_cube.wasm
cargo install simple-http-server
start firefox.exe "http://localhost:4000"
timeout /t 1 > nul
simple-http-server -i -b 127.0.0.1 -p 4000
