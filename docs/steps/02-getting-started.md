---
permalink: "/steps/02-getting-started.html"
title: "Rust Tools"
layout: "post"
prev: 
    url: "/steps/01-what_is_rust.html"
    text: "What is Rust"
next: 
    url: "/steps/02.1-rust-up.html"
    text: "Rustup"
readings:
    - url: "https://github.com/rust-lang-nursery/rustup.rs"
      text: "The Rustup Repo"
    - url: "https://doc.rust-lang.org/cargo/guide/"
      text: "The Cargo Guide"
---
- Rustup
    - Tool for managing the rust compiler's installation
- Cargo
    - A build and dependency management tool
    - Think NPM/Yarn

<div class="explain">
<p>The Rust team provides these two tools to make developer's lives easier. Rustup is a command line tool for managing the rust compiler's installation. </p>
<p>Rustup will allow you to update your version of the compiler, install different compiler targets (like building for Linux from OSX), and/or install multiple versions of the compiler (like stable, beta or nightly)</p>
<p>Cargo on the other hand is a wrapper around the rust compiler that allows for a much more user friendly interface. Instead of having to figure out which compiler flags to send, it defines a good set of defaults, lets you set some of them via a config file, and/or easily switch which compiler you are using. In addition to these great features it also manages your dependencies, downloading and compiling them as needed. The last great feature of cargo is that it will allow you to install binary items as well, compiling them for your computer and storing them in your path.</p>
</div>