---
title: Threading
subtitle: Guides
menu_position: 1
---


# The _sync_ flag 

You can enable the sync flag in your _cargo.toml_ file by changing the Arkham deceleration to:

```Toml
arkham = { version = "*", features=["sync"] }
```

With the _sync_ flag enabled `Res` and `State` will be thread safe. This makes it easy to pass the application state or resources to other threads for processing.

# Render signals

When manipulating data from outside of components, especially in other threads, it is useful to be able to notify the app instance that it needs to render changes to the screen. a `Renderer` provides the ability to signal the app instance that it needs to render.


```Rust
let mut app = App::new(root_view);
let renderer = app.get_renderer();
std::thread::spawn(move || loop {
    renderer.render()
    std::thread::sleep(
        std::time::Duration::from_secs(10)
    );
});
app.run();
```

# Full threading example

<CodeFile file="../examples/threading.rs" />




