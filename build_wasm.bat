cd crates/browser
cargo +nightly build --target wasm32-unknown-unknown
cd ../..
wasm-bindgen .\target\wasm32-unknown-unknown\debug\wasm_tutorial_browser.wasm --out-dir ./dist
