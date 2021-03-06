#! /bin/bash
if [[ $CARGO_HOME ]]; then 
    CARGOHOME=$CARGO_HOME
fi
if [[ ! $CARGOHOME ]]; then
    echo "CARGOHOME not set"
    CARGOHOME="$HOME/.cargo"
fi
if [ ! -f $CARGOHOME/bin/cargo-upstall ]; then
    echo "cargo-install does not exist"
    cargo install cargo-upstall
fi
if [ ! -f $CARGOHOME/bin/wbch ]; then
    echo "installing wasm/webpack/chrome hack"
    cargo install --git https://github.com/freemasen/wasm-chrome-hack
fi
echo "Installing nightly"
rustup install nightly
echo "Installing wasm32-unknown-unknown"
rustup target add --toolchain nightly wasm32-unknown-unknown
cd ./crates/browser
echo "building the browser project"
cargo +nightly build --target wasm32-unknown-unknown --release
cd ../..
echo "updating cargo upstall"
cargo upstall cargo-upstall
echo "installing wasm-bindgen"
cargo upstall wasm-bindgen-cli
echo "making sure that rsass exists"
cargo upstall rsass --features commandline
echo "running wasm-bindgen against our project"
wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm_tutorial_browser.wasm --browser --out-dir ./dist
echo "renaming the .wasm file generated by wasm-bindgen"
NAME=bincode_parse.wasm
mv ./dist/wasm_tutorial_browser_bg.wasm ./dist/$NAME
echo "running the chrome hack bin"
wbch ./dist/wasm_tutorial_browser.js ./ts/wasm_tutorial_browser.js /$NAME
echo "deleting the wasm-bindgen generated .js file"
rm ./dist/*.js
echo "running sass"
mkdir -p ./dist/css
rsass --style compressed ./sass/main.scss > ./dist/css/main.css
echo "downloading deps"
npm install
echo "running webpack"
./node_modules/.bin/webpack $1
