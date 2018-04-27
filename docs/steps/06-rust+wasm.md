---
permalink: "/steps/06-rust+wasm.html"
title: "Rust + WASM"
layout: "post"
prev: 
    url: "/steps/05.1-death.html"
    text: "Is JS Dying?"
next: 
    url: "/steps/08-hw.wasm.html"
    text: "Hello, WASM!"
---

## Options
### emscripten
- Originally LLVM to asm.js compiler
- Now can output as WASM
- Lots of extra stuff added
    - Glue code
    - c and c++ constructs (filesystem, etc)
<div class="explain">
Rust can be compiled using a <code>--target</code> argument, this argument takes a value describing where the bin/lib will actually be run. We can use the "triplet" <code>wasm32-unknown-emscripten</code> to generate this kind of module.
</div>

```bash
$ cargo build --target wasm32-unknown-emscipten
```

### wasm-bindgen
- Rust internal project
- Injects code into rust projects
- Generates `.js` glue code
<div class="explain">
Using the "triplet" <code>wasm32-unknown-unknown</code> we would generate a <code>.wasm</code> file that only has glue injected where we specify.
</div>

```bash
$ cargo build --target wasm32-unknown-unknown
```

<div class="explain">
We are going to use the latter option going forward, <a href="http://kripken.github.io/emscripten-site/docs/compiling/WebAssembly.html?highlight=wasm">click here</a> to learn more about the emscripten. I found the overall experience with the emscripten to be a little more difficult to understand.
</div>