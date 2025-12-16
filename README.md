
Pomodoro App:
https://pomodoro-rust.onrender.com


Build command:
rustup target add wasm32-unknown-unknown && cargo install wasm-pack --locked && wasm-pack build --target web && cd backend && cargo build --release

Run command:
cd backend && ./target/release/pomodoro-server
