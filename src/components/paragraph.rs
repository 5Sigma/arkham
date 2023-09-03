use std::ops::Deref;

use crossterm::style::Color;

use crate::prelude::{Callable, ToRuneExt};

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
            let l = line.deref().to_runes();

            stack.insert(line);
        }
        view.component(view.size(), stack);
    }
}
