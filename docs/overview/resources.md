---
title: Resources & State
subtitle: Overview
menu_position: 2
---

# Dependency Injection

Being able to access your own objects and state is 
important for any application. Arkham focuses heavily
on making this ergonomic and easy. 

There are two types of injectable objects a _State_ 
object and _Resource_ object. The main difference is 
Resource objects are provided immutable and State 
objects are provided with the ability to borrow both
as mutable and immutable references.

## Defining an injectable object

_Resources_ and _state_ are added during application startup
and are global to the application. A single state object can
be used to maintain the full state of the application and 
individual components can read and write to the sections they 
need.

Resources and state must have unique _Type_. Only one instance 
any type can be inserted.


```Rust

pub struct Person {
    pub name: String,
    pub age: u16
}

#[derive(Default)]
pub struct AppState {
    pub counter: usize,
}

let people: Vec<Person> = load_people();

App::new(root_view)
    .insert_resource(people)
    .insert_state(AppState::default())
    .run();

```

# Using resources and state

Injectables are provided automatically to any component that accepts them. 
Accepting them requires the use of a wrapper component depending on which 
it is.

- Resources use `Res&lt;T&gt;`
- State objects use `State&lt;T&gt;`

## Using a resource

Including the resource in the function arguments automatically provides 
the object inside a `Res` wrapper, which derefs to the underlying object.

```Rust
fn my_component(ctx: &mut ViewContext, people: Res<People>) {
    for (idx, person) in people.iter().enumerate() {
        ctx.insert((0, idx), person.name);
    }
}
```

## Using state


Including the resource in the function arguments automatically provides 
the object inside a `State` wrapper. The state wrapper has two primary
functions `State::get` which returns immutable access to the state
and `State::get_mut` which returns a mutable reference.

```Rust
fn my_component(ctx: &mut ViewContext, state: State<AppState>) {
    ctx.insert(
        (0, 0), 
        format!("Counter: {}", state.get().counter));
}
```

<Alert style="warning" title="Don't borrow state mutably more than once">
    Under the hood state is provided inside `Rc&lt;RefCell&lt;T&gt;&gt;`. 
    Take care not to call `State::get_mut`, which is effectively calling `RefCell::borrow_mut` more than 
    once at a time.
    
    This includes holding it and then calling a sub component that attempts to access state again. 
    Scope calls to `State::get_mut` so they live as short as possible and clone out objects if needed.
</Alert>

