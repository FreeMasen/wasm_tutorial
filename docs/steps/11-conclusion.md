---
permalink: "/steps/11-conclusion.html"
title: "Conclusion"
layout: "post"
prev:
    url: "/steps/10.13-build.html"
    text: "Build the app"
---
<div class="explain">
At this point, you should have a workable <code>WASM</code> project, though you would need to stand up a server to actually get it running. I have completed both sides of the implementation that you could play around with, to do that you would need to check out that repo and build it.
</div>

```bash
$ git clone https://github.com/freemasen/wasm_tutorial
$ cd wasm_tutorial
$ ./build_wasm.sh
$ cargo run
```

<div class="explain">
That set of commands will build both the wasm and server implementations, you could then open your browser to <a href="http://localhost:8888"><code>localhost:8888</code></a> and see the application working.
</div>