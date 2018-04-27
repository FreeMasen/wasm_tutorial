---
permalink: "/steps/04-hello-world-library.html"
title: "Hello Library!"
layout: "post"
prev: 
    url: "/steps/03-hello-world.html"
    text: "Hello World!"
next: 
    url: "/steps/04.2-using-lib.html"
    text: "Using the Library"
---
## Compartmentalize

<div class="explain">
<p>In addition to the binary application type rust also offers a library application type.</p>

<p>If we were starting from 0, we could use the command <code>cargo new --lib hello_world</code> to create a new library but for simplicity lets just add a library to this project. We can do that by adding a <code>lib.rs</code> file to the <code>src</code> folder.</p>
</div>

#### lib.rs
```rust
pub fn generate_greeting() -> String {
    "Hello, world!".to_string()
}

pub fn generate_custom_greeting(name: &str) -> String {
    format!("Hello, {}!", name);
}
```
<div class="explain">
now if we were to run <code>cargo build --lib</code> we would generate a library file that could be used from other <code>rust</code> programs.
</div>
<a class="explain" href="{{"/steps/04.1-strings.html" | relative_url}}">What is with the ! and calling .to_string() on a string?</a>