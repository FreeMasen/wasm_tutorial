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
    generate_custom_greeting("world")
}

pub fn generate_custom_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
```
<div class="explain">
<p>Rust has some strange syntax if you are coming from another popular language. First the return value for functions is declared with <code>-></code> followed by the name of the type it returns. The second and maybe more strange looking is that return values are implicitly the last line of a function with no semi-colon. </p>
<p>now if we were to run <code>cargo build --lib</code> we would generate a library file that could be used from other <code>rust</code> programs.</p>
</div>
<br />
<a class="explain" href="{{"/steps/04.1-strings.html" | relative_url}}">What is with the ! and calling .to_string() on a string?</a>