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
readings:
  - url: "https://hacks.mozilla.org/2017/07/creating-a-webassembly-module-instance-with-javascript/"
    text: "Lin Clark's intro to WASM 1"
  - url: "https://hacks.mozilla.org/2017/07/memory-in-webassembly-and-why-its-safer-than-you-think/"
    text: "Lin Clark's intro to WASM 2"
  - url: "https://hacks.mozilla.org/2017/07/webassembly-table-imports-what-are-they/"
    text: "Lin Clark's intro to WASM 3"

---
  - A new standard for the browser
  - Based on asm.js
  - Intermediate language
    - Compiling to a half way point
    - Think Java/C#
  - Strong static typing
  - No garbage collector (for now)

<div class="explain">

<p>If you are unfamiliar with asm.js I suggest you check out <a href="http://asmjs.org/">asmjs.org</a> or <a href="https://www.destroyallsoftware.com/talks/the-birth-and-death-of-javascript">better yet watch this somewhat silly talk about it</a>. Essentialy it is just the most performant sub-set of javascript.</p>

<p>To move things beyond what <code>asm.js</code> could do, WebAssembly or wasm will provide developers with a way to compile their code to an intermediate language (IL). If you have ever written, Java or C#, these languages compile down to an IL (Java's bytecode, .net's MSIL), this intermediate step is much faster to finish compiling at run time.</p>

<p>Most languages that utilize "Just-In-Time" compilation need to have a supervisor injected into them called the garbage collector. This supervisor just monitors the memory use for your application and automatically deallocates any data that is no longer required. JS has a garbage collector that makes life a lot easier for js developers, however the current state of wasm does not allow for a garbage collector.</p>

<p>When you compare the above list with our list about rust, there is a lot of overlap. Especially when you include the fact that <code>asm.js</code> is a Mozilla project as well.</p>
</div>