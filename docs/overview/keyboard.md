---
title: Keyboard Input
subtitle: Overview
menu_position: 3
---

# Reading keyboard input

Keyboard input is provided by a `Keyboard` resource. This is 
automatically available. To read input from keyboard events
accept the resource as a parameter for a component function
and check the current state.

```Rust
fn show_keypress(ctx &mut ViewContext, kb: Res<Keyboard>) {
    if let Some(c) = kb.char() {
        ctx.insert(0, format!("Key press: {}", c));
    }
}
```

## Reading modifier keys

Modifier key state is provided within the keyboard resource.


```Rust
fn check_key(ctx &mut ViewContext, kb: Res<Keyboard>) {
    if kb.char() == Some('d') && kb.control() {
        ctx.insert(0, "Key Pressed")
    } else {
        ctx.insert(0, "Key NOT Pressed")
    }
}
```


# Full Keyboard example

<CodeFile file="../examples/keyboard.rs" />
