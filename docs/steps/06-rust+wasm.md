---
permalink: "/steps/06-rust+wasm.html"
title: "Rust + WASM"
layout: "post"
prev: 
    url: "/steps/05.1-death.html"
    text: "Is JS Dying?"
next: 
    url: "/steps/07-hw.wasm.html"
    text: "Hello, WASM!"
readings:
    - url: "http://kripken.github.io/emscripten-site/docs/compiling/WebAssembly.html?highlight=wasm"
      text: "Emscripten's (an alternate wasm target) WASM page"
    - url: "https://medium.com/@ianjsikes/get-started-with-rust-webassembly-and-webpack-58d28e219635"
      text: "Rust+Emscripten introduction"
    - url: "https://github.com/rustwasm/wasm-bindgen"
      text: "wasm-bindgen"
---
<div class="presenting">
<h1>wasm-bindgen</h1>
<ul>
<li>
Rust internal project
</li>
<li>
Injects code into rust projects
</li>
<li>
Generates <code>.js</code> glue code
</li>
</ul>
</div>
<div class="explain">
<p>
We are going to use a project from Mozilla that is designed to make working with wasm in js possible. This project is called <code>wasm-bindgen</code>, bindgen is a portmanteau of bindings and generation, it will generate a javascript file that will "bind" to our wasm functions. The reason we can do this is because the kind folks that work on the Rust compiler have added the ability to build Web Assembly instead of a native application. We can tell cargo we want to do this by using the <code>--target</code> flag and then provide this "target".
</p>
<p>
The "target" flag allow us to compile programs that will run on another platform from whatever computer we are on. Say you wanted to compile a Linux program from your Mac, you can do that by telling the rust compiler to "target" linux. This is how we are going to go from Rust to Web Assembly, we are going to tell cargo that our "target" is <code>wasm32-unknown-unknown</code>. Targets have a special structure called a "triplet", each position means something different and there are three of them, fancy that.
</p>
<p>
The first value is going to be the instruction set to use, the most common would be "x86_64", some others you might know are "i686", "powerpc" or "arm". The second position is for the manufacturer, some you might know are "sun", "apple" or, for some reason "pc-windows". The last position in our triplet is going to be the operating system, some examples of those include "ios", "solaris", "fuchsia" or "redox". Putting that together we are going to be compiling for the "wasm32" instruction set and an "unknown" manufacturer and an "unknown" os, which checks out since we wont know the last two at all.
</p>
<p>
Rust does offer another wasm target <code>wasm32-unknown-emscripten</code>, this is absolutely an option, however I am not going to cover it in this tutorial.
</p>
</div>