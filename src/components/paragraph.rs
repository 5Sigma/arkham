use crate::prelude::{Callable, ToRuneExt};
use crossterm::style::Color;
use std::ops::Deref;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Paragraph {
    content: String,
    fg: Option<Color>,
    bg: Option<Color>,
}

impl Paragraph {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            fg: None,
            bg: None,
        }
    }
    pub fn height(&self, width: usize) -> usize {
        textwrap::wrap(&self.content, width).len()
    }
}

impl Callable<()> for Paragraph {
    fn call(&self, view: &mut crate::prelude::ViewContext, _args: ()) {
        let lines = textwrap::wrap(&self.content, view.width());
        let mut stack = view.vertical_stack(view.size());
        for line in lines.iter() {
            let _ = line.deref().to_runes();

            stack.insert(line);
        }
        view.component(view.size(), stack);
    }
}
