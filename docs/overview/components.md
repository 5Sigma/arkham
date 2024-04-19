---
title: Designing Components
subtitle: Overview
menu_position: 1
---


# Component overview

A component in Arkham is a piece of rendering logic that could be reusable, or could be used for organization. 
They are simple functions that accept a `ViewContext` reference, alongside any injected objects. See [Dependency Injection](resources)

A simple component might look like this:

```Rust
fn seperator(ctx: &mut ViewContext) {
    let width = ctx.size().width;
    let sep_chars = "-".repeat(ctx.size().width);
    ctx.insert(sep_chars);
}
```

## Components with parameters

Reusable components will need the ability to pass parameters into them. This is done by returning the component function from within another function. 

```Rust
fn name_field(name: &str) -> impl Fn(&mut ViewContext) {
    move |ctx:&mut ViewContext| {
        ctx.insert((0,0), "Name:");
        ctx.insert((10,0), name);
    }
}
```

## Understanding component sizing and positioning

When a component is used it is allocated a specific `Rect` that it can render to. 
The total dimensions for the component are available in `ViewContext::size()`. 
Components are also rendered at a specific position (the upper left corner). 
Inside a component its coordinates are relative to itself, its upper left corner is (0,0).


## Using components

A component can render other components inside them. 

In this example we can also see the component positioning and sizing. 
The first parameter to `ViewContext::component` is a Rect for the component.
This is the position in the parent component it will render its top left corner 
to and the size the component is allowed to render to.

The first parameter to `Rect::new` is the `Pos` for the rect, the coordinates 
of its top left corner. From the perspective of the `field` component, 
the content is inserted at y=0. However, when the component is placed in 
the `container` component, its upper left coordinate is specified and the 
coordinates of the `field` component become relative to the specified position 
in the `container` component.

The second parameter to `Rect::new` is the `Size` of the `Rect`. Its width and height.

```Rust
fn field(key: &str, value: &str) -> impl Fn(&mut ViewContext) {
    move |ctx: &mut ViewContext| {
        ctx.insert((0,0), format!("{}:", key));
        ctx.insert((10,0), value);
    }
}

fn container(ctx: &mut ViewContext) {
    ctx.component(Rect::new((0,0), (10,1)), field("Name", "Alice")
    ctx.component(Rect::new((0,1), (10,1)), field("Age", "22")
}

```

The `container` component will render to the following:

```
Name:    Alice 
Age:     22
```


## Components as closures

When you need something more complex than `ViewContext::insert`, but don't want to 
build a whole separate component. Components can be defined inline using a closure.


```Rust
fn container(ctx: &mut ViewContext) {
    let size = ctx.size();
    ctx.component(
        Rect::new((0,0), (size.width, size.height /2))
        |ctx: &mut ViewContext| {
            ctx.fill_all(Color::Blue);
        }
    );
}
```

