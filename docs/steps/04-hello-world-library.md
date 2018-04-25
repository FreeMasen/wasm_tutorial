---
permalink: "/steps/04-hello-world-library.html"
title: "Hello World Library"
layout: "post"
prev: "/steps/03-hello-world.html"
next: "/steps/04.2-using-lib.html"
---
## Compartmentialize

<div class="explain">

In addition to the binary application type rust also offers a library application type. 

If we were starting from 0, we could use the command `cargo new --lib hello_world` to create a new library application but for simplicity lets just add a library to this project. We can do that by adding a `lib.rs` file to the `src` folder.
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

now if we were to run `cargo build --lib` we would generate a library file that could be used from other `rust` programs. 

[What is with the ! and calling .to_string() on a string?](/steps/04.1-strings.html)
</div>