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
<p>Rust is a programming language that came out of the research department at Mozilla. It's main goal is to provide the performance of fully compiled languages like C and C++, while maintaining a memory guarantee that those languages cannot. It does this by adding a few extra restrictions to the compiler. What this means for developers is, your program wont run if you write code that might be result in invalid memory.</p>
<p>It features a strong, static type system, this means that any variable or return value must have a type declaration. The type system is quite robust and the compiler can infer almost all of your variable declarations. In the event you need something more flexable, it has both generic types and <code>enum</code> cases with associated values.</p>
</div>

<a class="explain" href="{{ "/steps/01.1-enum-generic.html" | relative_url}}">Generic? Associated value?</a>