---
permalink: "/steps/10-contrived-example.html"
title: "A Contrived Example"
layout: "post"
prev: "/steps/09-hw.js.html"
next: "/steps/10.1-the-wrench.html"
---
## Specification
### Main goal
Build a todo list application
### Routes
- GET: todos
    - gets all todo items
- POST: todos
    - Modify a single todo item
    - The body of this request should contain an object describing the action.

#### message.json
```json
{
    "kind": "Add|Remove|Update",
    "data": [{
        "id": 0,
        "complete": false,
        "action": "text here!"
    }]
}
```
### Design

![Design spec image]({{"/assets/img/design.png" | relative_url}})