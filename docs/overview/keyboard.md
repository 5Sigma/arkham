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

```Rust
use arkham::prelude::*;

fn main() {
    App::new(root).run().expect("couldnt launch app");
}

fn root(ctx: &mut ViewContext) {
    let size = ctx.size();
    ctx.fill(size, Rune::new().bg(Color::DarkGrey));
    ctx.component(((10, 10), (30, 1)), hello_world);
    ctx.component(((10, 11), (20, 1)), show_key_press);
    ctx.component((0, (size.width, 1)), quit_nag);
}

fn hello_world(ctx: &mut ViewContext) {
    ctx.insert(0, "Hello World, Press a key");
}

fn show_key_press(ctx: &mut ViewContext, kb: Res<Keyboard>) {
    if let Some(c) = kb.char() {
        ctx.insert(0, format!("Key press: {}", c));
    }
}

fn quit_nag(ctx: &mut ViewContext) {
    let size = ctx.size();
    ctx.insert(
        ((size.width / 2) - 7, 0),
        "Press Q to Quit".to_runes().fg(Color::Red),
    );
}
```
