---
permalink: "/steps/03-hello-world.html"
title: "Hello World"
layout: "post"
prev: "/steps/02.2-cargo.html"
next: "/steps/04-hello-world-library.html"
---
## Our first rust program
To get started we are going to use the `cargo new` command. This will generate all of your required project file and put them in the right place.
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

We get a new `Cargo.toml` file, a `src` folder and in that a `main.rs` file.
</div>

#### main.rs
```rust
fn main() {
    println!("Hello, world!");
}
```
<div class="explain">


Like `C` rust `binary (application)` projects require a function called `main`. For a rust program main needs to take no parameters and return nothing. Here we have a function that simply prints "Hello, world!" to the command prompt. lets test that out.
</div>

```bash
$ cargo run
   Compiling hello_world v0.1.0 (file:///mnt/c/Users/rmasen/Documents/projects/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 2.56 secs
     Running `target/debug/hello_world`
Hello, world!
```
