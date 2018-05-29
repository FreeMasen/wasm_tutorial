# wasm_tutorial

This project is used as a contrived example of how to use Web Assembly in the browser today.

[follow the tutorial](https://freemasen.github.io/wasm_tutorial)

If you are ready to see it in action the result is [hosted here](https://todo-wasm.herokuapp.com)

If you wanted to run it yourself you would need to do the following

```
git clone https://github.com/freemasen/wasm_tutorial
cd wasm_tutorial
./build.sh
cargo run
```

The above command requires `rustup` and `cargo` are installed and will install the `nightly` version of the compiler as well as the `wasm32-unknown-unknown` target as wells as a few command line tools via `cargo install`. In addition to the rust dependencies you would need `node`/`npm` installed and the project will be installing all of the dependencies, including `typescript` and `webpack`.

## contributing
If you have found a typo in the tutorial or an error in the code, please let me know by either opening an issue or pull request on this repo.