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
<div class="presenting">
<ul>
    <li>From Mozilla</li>
<li>Strong, static typing</li>
<li>Fully complied to native code
    <ul>
        <li>No VM</li>
    </ul>
</li>
<li>Memory safe guarantee
    <ul>
        <li>No garbage collector</li>
    </ul>
</li>
</ul>
</div>
<div class="explain">
<p>Rust is a programming language that came out of the research department at Mozilla. It's main goal is to provide the performance of fully compiled languages like C and C++, while maintaining a memory guarantee that those languages cannot. It does this by adding a few extra restrictions to the compiler. What this means for developers is, if your program compiles you shouldn't need to worry about runtime errors.</p>
<p>It features a strong, static type system, this means that any variable or return value must have a type declaration. That might seem like it would be painful to write however the compiler can infer the type of almost all of your variable just from the context. In the event you need something more flexible, it has both generic types and <code>enum</code> cases with associated values.</p>
</div>

<a class="explain" href="{{ "/steps/01.1-enum-generic.html" | relative_url}}">Generic? Associated value?</a>