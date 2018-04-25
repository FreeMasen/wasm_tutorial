@echo off
del %~dp0dist\*.wasm
cd crates\browser
cargo +nightly build --target wasm32-unknown-unknown
cd ..\..
wasm-bindgen ./target\wasm32-unknown-unknown\debug\wasm_tutorial_browser.wasm --browser --out-dir ./dist
echo "%~dp0wbch.exe"
if not exist %~dp0wbch.exe (
    echo "Cloning wbch into temp directory"
    git clone https://github.com/freemasen/wasm-chrome-hack %TEMP%\wasm_tut
    cd "%TEMP%\wasm_tut"
    cargo build --release
    cd %~dp0
    echo "Attempting to copy"
    robocopy %TEMP%\wasm_tut\target\release %~dp0 wbch.exe
    echo "Removing temp directory for repo"
    rmdir /S /Q %TEMP%\wasm_tut
)
echo "modifying file"
rename %~dp0dist\wasm_tutorial_browser_bg.wasm bincode_parse.wasm
wbch.exe ./dist/wasm_tutorial_browser.js ./ts/wasm_tutorial_browser.js /bincode_parse.wasm
del /F /Q %~dp0dist\*.js
%~dp0node_modules\.bin\webpack.cmd %1