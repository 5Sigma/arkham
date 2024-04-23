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

fn main() {
    App::new(root_view)
        .insert_resource(Theme::default())
        .insert_state(AppState::default())
        .run()
        .unwrap();
}

fn root_view(ctx: &mut ViewContext, kb: Res<Keyboard>, state: State<AppState>) {
    let size = ctx.size();

    ctx.fill_all(Color::Black);
    ctx.component(size, todo_container);

    if matches!(state.get().route, AppRoute::NewTodo(_)) {
        let mut rect = Rect::with_size(size);
        rect.pad(-5, -5);
        ctx.component(rect, add_todo_modal);
    }

    if kb.char() == Some('n') {
        state.get_mut().route = AppRoute::NewTodo(NewTodoRoute::default());
    }
}

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
                kb.reset();
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
