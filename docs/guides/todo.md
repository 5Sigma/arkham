---
title: Todo App
subtitle: Guides
---

# Obligatory todo app

This is a walk through for the obligatory Todo example that is required for all frameworks. This guide will setup a basic todo app in just over 100 lines of sparse code.

# Setting up the project

We will setup the project using _cargo new_. After these commands we 
should have basic Rust project with a _src/main.rs_ file. All further code 
will happen in _main.rs_.

```
cargo new todo
cd todo
cargo add arkham
```

# Some basic structures

We will need a few structures. The first is `AppRoute`, which will tell us which _view_ we are in. For this simple application we will have two options either the _TodoPage_ view, or the _NewTodo_ view.

We will another routing structure for a sub route called `NewTodoRoute`. This isn't strictly necessary since it will only have one option _Title_.
Third, a `Todo` structure which will be our data structure for holding individual todo items. And last, a structure to hold the overall application state.

```Rust
use arkham::prelude::*;

#[derive(Default, Clone, Copy)]
enum AppRoute {
    #[default]
    TodoPage,
    NewTodo(NewTodoRoute),
}

#[derive(Default, Clone, Copy)]
enum NewTodoRoute {
    #[default]
    Title,
}

#[derive(Default)]
struct Todo {
    title: String,
    complete: bool,
}

#[derive(Default)]
struct AppState {
    todos: Vec<Todo>,
    selected_index: usize,
    route: AppRoute,
    new_todo_form: Todo,
}
```

# The main function

The main function is very simple. We instantiate the `Arkham::App` and insert two resources: A theme and the application state.
We also pass the `root_view` function to the application as the initial rendering code that app will start with. 
This function will be defined next.

```Rust
fn main() {
    App::new(root_view)
        .insert_resource(Theme::default())
        .insert_state(AppState::default())
        .run()
        .unwrap();
}

```

# The root view

The root view is also fairly straightforward. This is a _component_. Component's in Arkham are simple functions that accept a 
mutable instance of `ViewContext`. They can further accept other resources and states that were inserted during `App` creation; 
such as `Theme` and `AppState`. For more information on components see [Designing Components](/overview/components).

This component receives two injected objects. `Res<Keyboard>`, which denotes the current keyboard state, and `AppState`, which is 
our application state object we inserted in `main`. For more information on resources and states see [Dependency injection](/overview/resources).

## The code

First we extract the size from the context, in order to evade borrow rules when we pass it into `ViewContext::componet`. This is the available size we have to work with. Because this is the root component, the size is the total available space in the terminal.

Next we fill the entire area with black, and insert the `todo_container` component. Which is a function we will define later.

Then we match the current route to see if the route is set to `AppRoute::NewTodo`, if it is we can render the `add_todo_modal`.

Last we check the keyboard state to see if _n_ was pressed, if so we will update the route to  `AppRoute::NewTodo`.

<Alert style="info">
The order here is important. We want to check the keyboard state after we possibly render the new todo form. This is because we
want to give `new_todo_modal` a chance to intercept the keyboard input. 
</Alert>

```Rust
fn root_view(
    ctx: &mut ViewContext, 
    kb: Res<Keyboard>, 
    state: State<AppState>) {
        let size = ctx.size();

        ctx.fill_all(Color::Black);
        ctx.component(size, todo_container);

        if matches!(state.get().route, AppRoute::NewTodo(_)) {
            let mut rect = Rect::with_size(size);
            rect.pad(-5, -5);
            ctx.component(rect, add_todo_modal);
        }

        if kb.char() == Some('n') {
            state.get_mut().route = 
                AppRoute::NewTodo(NewTodoRoute::default());
        }
}
```

# Todo container

The todo container's code could simply be in the root_view, but we will break it out under the assumption that we might want to add more 
functionality and views later. This component receives the same two inserted objects as before: The keyboard resource 
and the application state.


## Code

First we retrieve a reference to the app state, which we will use throughout. Then we start doing keyboard handling:

- *space* will toggle the _complete_ status of the currently selected todo.
- *k* or *down arrow* will move the current selection down
- *j* or *up arrow* will move the current selection up
- *x* or *delete* will delete the currently selected item

After handling keyboard events we just loop through all the todo items in the app state and use the `todo` component for each one.


```Rust
fn todo_container(ctx: &mut ViewContext, state: State<AppState>, kb: Res<Keyboard>) {
    {
        let mut st = state.get_mut();
        if kb.char() == Some(' ') {
            let idx = st.selected_index;
            if let Some(todo) = st.todos.get_mut(idx) {
                todo.complete = !todo.complete;
            }
        }
        if (kb.char() == Some('k') || kb.code() == Some(KeyCode::Up)) && st.selected_index > 0 {
            st.selected_index -= 1;
        }

        if (kb.char() == Some('j') || kb.code() == Some(KeyCode::Down))
            && st.selected_index < st.todos.len() - 1
        {
            st.selected_index += 1;
        }

        if kb.code() == Some(KeyCode::Delete) || kb.char() == Some('x') {
            let idx = st.selected_index;
            st.todos.remove(idx);
            if st.selected_index > st.todos.len() - 1 && !st.todos.is_empty() {
                st.selected_index = st.todos.len() - 1;
            }
        }
    }

    for idx in 0..(state.get().todos.len()) {
        let size = ctx.size();
        ctx.component(((0, idx), (size.width, 1)), todo(idx));
    }
}
```

# Todo component

The todo component simply prints out the todo item on a line. It styles the text so that the color changes if it 
is the current selection, or if it is completed.

This code uses the `Stack` component to make this easier. For more information see [Stacks](/overview/stacks).

```Rust
fn todo(todo_index: usize) -> impl Fn(&mut ViewContext, Res<Theme>, State<AppState>) {
    move |ctx, theme, state| {
        let st = state.get();
        let size = ctx.size();
        let todo = st.todos.get(todo_index).unwrap();
        let fg = if todo.complete {
            Color::DarkGrey
        } else {
            theme.fg
        };
        if st.selected_index == todo_index {
            ctx.fill_all(theme.bg_selection);
        }
        let mut stack = ctx.horizontal_stack(size);
        if todo.complete {
            stack.insert("  [x] ".to_runes().fg(fg));
        } else {
            stack.insert("  [ ] ".to_runes().fg(fg));
        }
        stack.insert(todo.title.to_runes().fg(fg));
        ctx.component(size, stack);
    }
}
```

# The new todo modal

Last we have the todo modal, which has the most complexity. This renders a pane in the center of the screen and acts as a _form_. 
For this example there is only a single _title_ field.

When we rendered this component above we specifically set its `Rect` to shrink by 5 characters 
allowing it to not fill the screen completely. From within this component we can operate normally 
since the `ViewContext::size` will already have that factored in.

The `AppState` has a member called new_title_form which is a `Todo` that will hold the data for a todo that is 
currently being edited here.

## The code

Same as before we copy the size from the context to use it later. We also pull out the _sub route_. 
If we had more fields, like a due date, we could use this sub-route to determine which _field_ had focus. For us it will always 
be the _title field_.

Next we fill the first line of the area with the tertiary color, just to give a nice title bar for the modal, and insert some 
title text.

We check for the _Escape_ key to possibly close the modal if it is pressed.  We could check for this in the todo container, but it is 
nice to have all the code related to this component included inside it. This way no matter where we use this component the escape key 
will always just work.

The last thing we render is the title field itself. This is just the text _Title_, then an area filled in with the tertiary color
and has the current title value printed in it. 

Now we handle the text input. We match on the route, again in case we had multiple fields, and check for any character press.
if we have one we push it into the title value. 

We also check for a _enter_ key press. When enter is pressed we will push the `new_todo_form` todo data into the todos `Vec`.
and change the route.

Last we reset the keyboard. This is important because we want all the character presses here.
We do not want components down the line to see these keyboard events. If we did not do this, when we used the _n_ key in a title it would 
trigger the _opening modal logic_ we defined above. Also the _q_ key would quit the application, when it was pressed while editing the title.


```Rust
fn add_todo_modal(
    ctx: &mut ViewContext,
    theme: Res<Theme>,
    state: State<AppState>,
    kb: Res<Keyboard>,
) {
    let size = ctx.size();

    let AppRoute::NewTodo(route) = state.get().route else {
        return;
    };

    ctx.fill_all(theme.bg_secondary);
    ctx.fill((0, (size.width, 1)), theme.bg_tertiary);
    ctx.insert((2, 0), "New Todo Item".to_runes().bold());

    if kb.code() == Some(KeyCode::Esc) {
        state.get_mut().route = AppRoute::TodoPage;
        kb.reset();
        return;
    }

    ctx.component((2, (size.width - 2, 1)), |ctx: &mut ViewContext| {
        let size = ctx.size();
        ctx.insert(0, "Title");
        ctx.fill(((10, 0), (size.width - 12, 1)), theme.bg_tertiary);
        ctx.insert((10, 0), state.get().new_todo_form.title.clone())
    });

    match route {
        NewTodoRoute::Title => {
            if let Some(c) = kb.char() {
                state.get_mut().new_todo_form.title.push(c);
            }
            if Some(KeyCode::Backspace) == kb.code() {
                state.get_mut().new_todo_form.title.pop();
            }
            if Some(KeyCode::Enter) == kb.code() {
                let mut st = state.get_mut();
                let todo = std::mem::take(&mut st.new_todo_form);
                st.todos.push(todo);
                st.route = AppRoute::TodoPage;
            }
        }
    }

    kb.reset();
}
```

And that's it, a basic Todo application. 

# Full example

Here is the full code put altogether:


<CodeFile file="../examples/todo.rs" />
