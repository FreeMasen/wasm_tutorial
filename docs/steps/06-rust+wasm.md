---
permalink: "/steps/06-rust+wasm.html"
title: "Rust + WASM"
layout: "post"
prev: 
    url: "/steps/05.1-death.html"
    text: "Is JS Dying?"
next: 
    url: "/steps/07-hw.wasm.html"
    text: "Hello, WASM!"
readings:
    - url: "http://kripken.github.io/emscripten-site/docs/compiling/WebAssembly.html?highlight=wasm"
      text: "Emscripten's WASM page"
    - url: "https://medium.com/@ianjsikes/get-started-with-rust-webassembly-and-webpack-58d28e219635"
      text: "Rust+Emscripten introduction"
    - url: "https://github.com/rustwasm/wasm-bindgen"
      text: "wasm-bindgen"
---

## Options
### emscripten
- Originally LLVM to asm.js compiler
- Now can output as WASM
- Lots of extra stuff added
    - Glue code
    - c and c++ constructs (file system, etc)
<div class="explain">
Rust can be compiled to this "target" by passing cargo the <code>--target</code> flag with an argument describing where the code will actually be run, called a triplet. We can use the triplet <code>wasm32-unknown-emscripten</code> to generate this kind of module.
</div>

```bash
$ cargo build --target wasm32-unknown-emscipten
```

### wasm-bindgen
- Rust internal project
- Injects code into rust projects
- Generates <code>.js</code> glue code
<div class="explain">
Using the triplet <code>wasm32-unknown-unknown</code> we would generate a <code>.wasm</code> file that only has no glue. We can use <code>wasm-bindgen</code> to inject the glue we are actually using.
</div>

```bash
$ cargo build --target wasm32-unknown-unknown
```

<div class="explain">
We are going to use the latter option going forward, I found the overall experience with the emscripten to be a little more difficult to understand. Your mileage may vary.
</div>