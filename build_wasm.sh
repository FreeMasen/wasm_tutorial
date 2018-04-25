#!/bin/bash
cd crates/browser
cargo +nightly build --target wasm32-unknown-unknown
cd ../..
wasm-bindgen ./target/wasm32-unknown-unknown/debug/wasm_tutorial_browser.wasm --browser --out-dir ./dist

if [ ! -f ./wbch ]; then
    echo "creating temp"
    CURDIR=$PWD
    echo $CURDIR
    TEMP=$(mktemp)
    echo $TEMP
    echo "removing temp file"
    rm -r $TEMP
    echo "cloning into temp folder"
    git clone https://github.com/freemasen/wasm-chrome-hack $TEMP
    echo "moving into temp directory"
    cd $TEMP
    echo "building project"
    cargo build --release
    echo "copying bin"
    cp ./target/release/wbch $CURDIR
    echo "move back to project folder"
    cd $CURDIR
    echo "removing temp dir"
    rm -rf $TEMP
    echo "chmod +x ing bin"
    chmod +x ./wbch
fi
echo "modifying file"
NAME=bincode_parse.wasm
mv ./dist/wasm_tutorial_browser_bg.wasm ./dist/$NAME
./wbch ./dist/wasm_tutorial_browser.js ./ts/wasm_tutorial_browser.js /$NAME
rm ./dist/*.js
webpack $1
