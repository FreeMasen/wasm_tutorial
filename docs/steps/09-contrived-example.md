---
permalink: "/steps/09-contrived-example.html"
title: "A Contrived Example"
layout: "post"
prev: 
    url: "/steps/08-hw.js.html"
    text: "Using the Bindings"
next: 
    url: "/steps/09.1-the-wrench.html"
    text: "The Wrench"
---
<div class="explain">
With <code>Hello, World!</code> out of the way, let's sink our teeth into something a little more interesting.
</div>
## Specification
### Main goal
Build a todo list application

<div class="explain">
The project manager has requested a todo list application, it should be as simple as possible.
</div>

### Routes
<div class="explain">
One route that will be made available with both a <code>GET</code> and and <code>POST</code> action.
</div>
- GET: todos
    - gets all todo items
- POST: todos
    - Modify a single todo item

#### todo.json
<div class="explain">
The primary model we are going to be using will be a <code>ToDo</code>, it should look like the following.
</div>

```json
{
    "id": 0,
    "complete": false,
    "action": "text here!"
}
```
<div class="explain">
<p>A <code>ToDo</code> has 3 properties</p>
<ul>
<li><code>id</code>: The numeric id,  -1 for new items</li>
<li><code>complete</code>: A boolean describing the completeness of the todo</li>
<li><code>action</code>: A string describing the activity that needs to be performed</li>
</ul>
</div>

### Design

![Design spec image]({{"/assets/img/design.png" | relative_url}})

<div class="explain">
The UX designer has provided you with the above wireframe to build the UI against
</div>