---
permalink: "/steps/05-what-is-wasm.html"
title: "What is WASM"
layout: "post"
prev: 
  url: "/steps/04.2-using-lib.html"
  text: "Using the Library"
next: 
  url: "/steps/05.1-death.html"
  text: "Is JS Dying?"
---
  - Based on asm.js
  - A new standard for running code in the browser
  - Intermediate language
    - Compiling to a half way point
    - Think Java/C# 
  - No garbage collector (for now)

<div class="explain">

<p>If you are unfamiliar with asm.js I suggest you check out <a href="http://asmjs.org/">asmjs.org</a> or <a href="https://www.destroyallsoftware.com/talks/the-birth-and-death-of-javascript">better yet watch this talk about the "future" of computing</a>. Essential it is just the most performant sub-set of javascript. </p>

<p>To move things even further forward, WebAssembly or WASM will provide developers with a way to compile their code to an intermediate language (IL). If you have ever written, Java or C#, these languages compile down to an IL (Java's bytecode, .net's MSIL), this intermediate step is much faster to finish compiling at run time. </p>

<p>Most languages that utilize runtime compilation need to have a supervisor injected into them called the garbage collector. This supervisor just monitors the memory use for your application and automaticlly deallocates any data that is no longer required. Javascript has a garbage collector that makes life a lot easier for us users, however the current state of WASM does not provide garbage collection.</p>
</div>