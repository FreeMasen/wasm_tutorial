---
permalink: "/steps/01-what_is_rust.html"
title: "What is Rust"
layout: "post"
prev: 
    url: "/"
    text: "Table of Contents"
next: 
    url: "/steps/02-getting-started.html"
    text: "Getting Started"
readings:
    - url: "https://www.rust-lang.org/en-US/faq.html"
      text: "The Rust FAQ"
---
- From Mozilla
- Strong, static typing
- Fully complied to native code
    - No VM
    - No garbage collector
- Memory safe guarantee

<div class="explain">
Rust is a programming language that came out of the research department at Mozilla. It's main goal is to provide the performance of fully compiled languages like C and C++, while maintaining a memory guarantee that those languages cannot. It does this by adding a few extra restrictions to the compiler, meaning you can't even get your program to run if you try to do something that might be result in invalid memory.
</div>