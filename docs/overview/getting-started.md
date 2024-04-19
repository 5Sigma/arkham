---
title: Getting Started
subtitle: Overview
menu_position: 0
---

# Starting a new project

To setup a new project use `cargo new` and `cargo add arkham` to add the dependency.

```Shell
cargo new my_project
cd my_project
cago add arkham
```

# Import the arkham prelude

Add the following line to the top of _my_project/src/main.rs_ to import all the Arkham members.

```rust 
use arkham::prelude::*;
```

# Setup the root view

Arkham requires a _Root View_ component that will act as a container view for the application. Views in Arkham are simple functions. 
We can add a root view function to _my_project/src/main.rs_
A simple root view may look like this:

```Rust
fn root_view(ctx &mut ViewContext) {
    ctx.insert((5,5), "Hello World");
}
```

# Setup the application

In our `main` function we can setup the application and run it, beginning the run loop. Replace the main function in _my_project/src/main.rs_ with the following:

```Rust
fn main() {
    App::new(root_view).run().unwrap();
}
```


# The full main.rs

The full main.rs now looks like this:

```Rust
use arkham::prelude::*;

fn main() {
    App::new(root_view).run().unwrap();
}

fn root_view(ctx &mut ViewContext) {
    ctx.insert((5,5), "Hello World");
}
```


This can now be run with `cargo run` and a hello world app, complete with a default 'q' hotkey to quit, will run.
