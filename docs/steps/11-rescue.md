---
permalink: "/steps/11-rescue.html"
title: "WASM to the rescue"
layout: "post"
prev: "/steps/10.1-the-wrench.html"
next: "/steps/11.1-define.html"
---
<div class="explain">
Since there is a rust library for serializing both Bincode and JSON, WASM can help. To get started we need to create a new library.
</div>

#### Generate a new library project
```bash
$ cargo new --lib bincode_parser
$ cd bincode_parser
```

<div class="explain">
Now that we have our project, we need to add some dependencies.
</div>

#### Cargo.toml
```toml
[package]
name = "bincode_parser"
version = "0.1.0"
authors = ["you"]

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = "*"
serde_derive = "*"
serde_json = "*"
bincode = "1.0.0"
wasm-bindgen = "0.2"
wasm_tutorial_shared = {path = "../shared}
```
<div class="explain">
<p>The main serialization library used in rust is called <code>serde</code>, short for (ser)ialize/(de)serialize. We also want to grab a special macro tied to that library called serde_derive. <code>Serde</code> doesn't implement any actual serialization or deserialization but depends on other to provide that implementations, <code>serde_json</code> and <code>bincode</code> do just that. We dont' want to forget <code>wasm-bindgen</code> and obviously the shared portion of the server that implements the models that will be sent.</p>
<p>At this point we can build a WASM module to convert these models from bincode to JSON.</p>
</div>
