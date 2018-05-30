---
permalink: "/steps/03-hello-world.html"
title: "Hello World!"
layout: "post"
prev: 
    url: "/steps/02.2-cargo.html"
    text: "Cargo"
next: 
    url: "/steps/04-hello-world-library.html"
    text: "Hello Library!"
readings:
    - url: "https://doc.rust-lang.org/book/second-edition/index.html"
      text: "The Rust Book"
---
## Our first rust program
<div class="explain">
To get started we are going to use the <code>cargo new</code> command. This will generate all of the required project files and put them in the right place.
</div>

```bash
$ cargo new hello_world
    Created binary (application) `hello_world` project
$ cd hello_world
$ tree
.
├── Cargo.toml
└── src
    └── main.rs
```
<div class="explain">
We get a new <code>Cargo.toml</code> file, a <code>src</code> folder and in that a <code>main.rs</code> file.
</div>

#### main.rs
```rust
fn main() {
    println!("Hello, world!");
}
```
<div class="explain">


Like <code>C</code> programs, rust programs require a function called <code>main</code>. For a rust program to work <code>main</code> needs to take no parameters and return nothing (there are a few things it can return but let's just stick with this for now). Here we have a function that simply prints "Hello, world!" to the command prompt; lets test that out. To do that we are going to use the command <code>cargo run</code>, which will compile our program and then run it, the program will be compiled into a <code>./target/debug/</code>folder.
<br />
</div>

```bash
$ cargo run
   Compiling hello_world v0.1.0 (file:///mnt/c/Users/rmasen/Documents/projects/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 2.56 secs
     Running `target/debug/hello_world`
Hello, world!
```
<div class="explain">
It worked!
</div>