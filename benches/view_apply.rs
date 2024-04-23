use std::{cell::RefCell, rc::Rc};

use arkham::internal::{Container, View};
use arkham::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn view_apply(views: &[View]) {
    let mut ctx = ViewContext::new(
        Rc::new(RefCell::new(Container::default())),
        (100, 100).into(),
    );
    for view in views {
        ctx.apply(0, view);
    }
}

fn bench_view_apply(c: &mut Criterion) {
    let mut views = vec![];
    views.push({
        let mut view = View::new((100, 100));
        view.fill_all(Color::Red);
        view.insert((10, 20), "Some test runes".to_runes().fg(Color::Blue));
        view
    });
    views.push({
        let mut view = View::new((100, 100));
        view.fill(((50, 50), (20, 20)), Color::Green);
        view
    });
    views.push({
        let mut view = View::new((100, 100));
        view.fill(((0, 99), (100, 1)), Color::Green);
        view
    });
    views.push({
        let mut view = View::new((100, 100));
        view.fill(((30, 0), (1, 99)), Color::DarkBlue);
        view
    });
    c.bench_function("View apply", |b| b.iter(|| view_apply(black_box(&views))));
}

criterion_group!(benches, bench_view_apply);
criterion_main!(benches);
