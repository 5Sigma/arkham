---
title: Stacks
subtitle: Overview
menu_position: 4
---

# Stack Components

Stacks are a convenient layout component available through the `ViewContext`. 
They allow a number of components to be easily positioned next to each other.

Stacks can be either _horizontal_ or _vertical_.

To use a stack initialize it from the context and use the `Stack::insert` 
and `Stack:component` functions. The `Stack::component` function requires only a 
`Size` and not a `Rect` like the `ViewContext`. This is because the stack will
automatically handle its positioning.


```Rust
let size = ctx.size();
let mut stack = ctx.vertical_stack(Size::new(100, 100));
stack.component((size.width, 2), my_component);
// We can pass size here and it will be automtically 
// converted to a Rect with position (0,0).
ctx.component(size, stack);
```

## Alignment

Stacks can also have a specified alignment. This will modify the 
positioning of the sub components so they align in the given direction.

Vertical stacks can have Left, Center, Right alignments
Horizontal stacks can have Top, Center, Bottom alignments

```Rust
let size = ctx.size();
let mut stack = ctx.vertical_stack(Size::new(100, 100));
stack.alignment(StackAlignment::Center);
```

# Full stack example

<CodeFile file="../examples/stack.rs" />
