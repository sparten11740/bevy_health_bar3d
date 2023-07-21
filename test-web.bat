cargo build --example dinosaurs --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/examples/dinosaurs.wasm
serve -s .