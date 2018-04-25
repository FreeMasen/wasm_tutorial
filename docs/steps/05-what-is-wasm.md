---
permalink: "/steps/05-what-is-wasm.html"
title: "What is WASM"
layout: "post"
prev: "/steps/04.2-using-lib.html"
next: "/steps/05-death.html"
---
  - Based on asm.js
  - A new standard for running code in the browser
  - Intermediate language
    - Compiling to a half way point
    - Think Java/C# 
  - No garbage collector (for now)

<div class="explain">

If you are unfamiliar with asm.js I suggest you check out [asmjs.org](http://asmjs.org/) or [better yet watch this talk about the "future" of computing](https://www.destroyallsoftware.com/talks/the-birth-and-death-of-javascript). Essential it is just the most performant sub-set of javascript. 

To move things even further forward, WebAssembly or WASM will provide developers with a way to compile their code to an intermediate language (IL). If you have ever written, Java or C#, these languages compile down to an IL (Java's bytecode, .net's MSIL), this intermediate step is much faster to finish compiling at run time. 

Most languages that utilize runtime compilation need to have a supervisor injected into them called the garbage collector. This supervisor just monitors the memory use for your application and automaticlly deallocates any data that is no longer required. Javascript has a garbage collector that makes life a lot easier for us users, however the current state of WASM does not provide garbage collection.
</div>