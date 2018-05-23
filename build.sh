#! /bin/bash
echo "Installing nightly"
~/.cargo/bin/rustup install nightly
echo "Installing wasm32-unknown-unknown"
~/.cargo/bin/rustup target add --toolchain nightly wasm32-unknown-unknown
cd ./crates/browser
echo "building the browser project"
~/.cargo/bin/cargo +nightly build -q --target wasm32-unknown-unknown
cd ../..
echo "installing wasm-bindgen"
~/.cargo/bin/cargo install wasm-bindgen-cli
echo "running wasm-bindgen against our project"
~/.cargo/bin/wasm-bindgen ./target/wasm32-unknown-unknown/debug/wasm_tutorial_browser.wasm --browser --out-dir ./dist
cd ./dist
ls
cd ..
echo "making sure that the wasm-bindgen-chrome-hack exists"
if [ ! -f ./wbch ]; then
    echo "creating temp"
    CURDIR=$PWD
    echo $CURDIR
    TEMP=$(mktemp)
    echo $TEMP
    rm -r $TEMP
    echo "cloning into temp folder"
    git clone https://github.com/freemasen/wasm-chrome-hack $TEMP
    echo "moving into temp directory"
    cd $TEMP
    echo "building project"
    ~/.cargo/bin/cargo build -q --release
    echo "copying bin"
    cp ./target/release/wbch $CURDIR
    echo "move back to project folder"
    cd $CURDIR
    echo "removing temp dir"
    rm -rf $TEMP
    echo "chmod +x ing bin"
    chmod +x ./wbch
fi
echo "renaming the .wasm file generated by wasm-bindgen"
cd ./dist
ls
cd ..
NAME=bincode_parse.wasm
mv ./dist/wasm_tutorial_browser_bg.wasm ./dist/$NAME
echo "running the chrome hack bin"
./wbch ./dist/wasm_tutorial_browser.js ./ts/wasm_tutorial_browser.js /$NAME
echo "deleting the wasm-bindgen generated .js file"
rm ./dist/*.js
echo "downloading deps"
npm install
echo "running sass"
./node_modules/.bin/node-sass --output-style compressed -o ./dist/css -x ./sass/main.scss
echo "running webpack"
./node_modules/.bin/webpack $1
