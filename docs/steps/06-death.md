---
permalink: "/steps/05-death.html"
title: "Is Javascript Dead or Dying?"
layout: "post"
prev: "/steps/04.2-using-lib.html"
next: "/steps/07-rust+wasm.html"
---
Not any time soon.

<div class="explain">

WASM was adopted as a minimum viable product in June 2015, almost 3 years later there is still a lot to be done. 

Currently WASM doesn't have access to the DOM, garbage collection, or access to system information (like <code>Date</code>). This means that the only way to interact with the user's experience is through a connection to the <code>javascript</code> context. Currently the only information that can pass from one context to the other is <code>number</code>s, the rest has to be pushed into shared memory and then the location and size of that memory can be passed between. This makes for a pretty complicated system.

While there may be an opportunity to move past these current limitations, it is going to take a long time to get there. Currently the [Webassembly Roadmap](http://webassembly.org/roadmap/) is almost all about stabilization and infrastructure and while the post-mvp features are listed as "in-progress" none of those are about `DOM` interaction.
</div>