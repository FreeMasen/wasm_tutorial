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
<div class="presenting">
<ul>
<li>A new standard for the browser</li>
<li>Based on asm.js</li>
<li>Intermediate language
  <ul>
    <li>Compiling to a half way point</li>
    <li>Think Java/C#</li>
  </ul>
</li>
<li>Strong static typing</li>
<li>No garbage collector (for now)</li>
</ul>
</div>
<div class="explain">
<p>Web Assembly or wasm is a new code standard for the browser. It is based on something called <code>asm.js</code></p>
<p>If you are unfamiliar with asm.js I suggest you check out <a href="http://asmjs.org/" target="self">asmjs.org</a> or <a href="https://www.destroyallsoftware.com/talks/the-birth-and-death-of-javascript" target="self">better yet watch this somewhat silly talk about it</a>. Essentially it is just the most performant sub-set of javascript.</p>

<p>To move things beyond what <code>asm.js</code> could do, WebAssembly or wasm will provide developers with a way to compile their code to an intermediate language (IL). If you have ever written, Java or C#, these languages compile down to an IL (Java's bytecode, .NET's MSIL), this intermediate step is much faster to finish compiling at run time.</p>

<p>Most languages that utilize "Just-In-Time" compilation or interpretation need to have a supervisor injected into them called the garbage collector. This supervisor just monitors the memory use for your application and automatically cleans up anything that you aren't using, JS has a garbage collector that makes life a lot easier. I only bring this up because wasm does not allow for a garbage collector, that may change in the future but currently we need to make sure we clean up our own garbage.</p>

<p>When you compare wasm with rust, there is a lot of overlap. Especially when you include the fact that <code>asm.js</code> is a Mozilla project as well.</p>
</div>