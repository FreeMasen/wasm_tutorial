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
<p>In that file we are going to write our first two rust functions, they'll look like this.</p>
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
<p>
Rust's syntax can seem strange if you are coming from most other languages, so what does all of that mean up there.
</p>
<p>
We use the keyword <code>pub</code> to make sure that this function will be available from outside of our library and then the keyword <code>fn</code> which is short for function. Next we give our function a name and then define any parameters, that part shouldn't be all that strange. Our arguments are going to be defined with a name then a colon and then the type, if you have more than one parameter you can just separate them with a comma like you would expect. The last thing to add to complete a function signature is the return type, to do this we use the thin arrow (<code>-></code>) followed by the type the function returns, if you aren't returning anything you can omit this part (no need to return <code>void</code> or anything like that). One of the strangest looking things about rust functions is that return values are the last line of a function's body with no semi-colon. So looking over the above code, we have two functions: <code>generate_custom_greeting</code> will take in a string and then replace the curly braces in "Hello, {}!" with that string returning the result, while <code>generate_greeting</code> will use <code>generate_custom_greeting</code> to create the string "Hello, world!" and return that.
</p>
<p>Now if we were to run <code>cargo build --lib</code> we would generate a library file that could be used from other <code>rust</code> programs, but how would we use that?</p>
</div>
<br />
<a class="explain" href="{{"/steps/04.1-strings.html" | relative_url}}">What is with the ! and calling .to_string() on a string?</a>