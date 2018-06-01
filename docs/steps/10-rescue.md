---
permalink: "/steps/10-rescue.html"
title: "WASM to the Rescue"
layout: "post"
prev: 
    url: "/steps/09.1-the-wrench.html"
    text: "The Wrench"
next: 
    url: "/steps/10.0-the-models.html"
    text: "The Models"
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
Now that we have our project, we need to add some dependencies. Update the <code>[dependencies]</code> section of your <code>Cargo.toml</code> file to include these items. While we are at it lets make sure that the <code>[lib]</code> section has <code>crate-type = ["cdylib"]</code> listed in it. It also might be a good idea to set <code>lto = true</code> in the <code>[profile.release]</code> and <code>[profile.dev]</code> sections, this will help to reduce our final <code>.wasm</code> file size by removing any unused glue code automatically.
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
wasm_tutorial_shared = { path = "../shared" }

[profile.release]
lto = true
[profile.dev]
lto = true
```
<div class="explain">
<p>The main serialization library used in rust is called <code>serde</code>, short for <strong>ser</strong>ialize/<strong>de</strong>serialize. We also want to grab a special macro tied to that library called <code>serde_derive</code>. <code>Serde</code> doesn't implement any actual serialization or deserialization but creates a baseline for developers to create implementations, <code>serde_json</code> and <code>bincode</code> do just that. We don't want to forget <code>wasm-bindgen</code> and obviously the models that our server developer shared with us.</p>

<p>Let's see what these models look like.</p>
</div>
