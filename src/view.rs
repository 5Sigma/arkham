use crate::{
    geometry::{Pos, Rect, Size},
    runes::{Rune, Runes},
};

#[derive(Clone)]
pub struct View(pub Vec<Vec<Rune>>);

impl std::ops::DerefMut for View {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for View {
    type Target = Vec<Vec<Rune>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl View {
    pub fn new<T>(size: T) -> Self
    where
        T: Into<Size>,
    {
        let size: Size = size.into();
        Self(vec![vec![Rune::default(); size.width]; size.height])
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<Rune>> {
        self.0.iter()
    }

    pub fn apply<P: Into<Pos>>(&mut self, pos: P, view: View) {
        let pos = pos.into();
        for (y, line) in view.0.iter().enumerate() {
            if self.0.len() > y + pos.y {
                for (x, rune) in line.iter().enumerate() {
                    if rune.content.is_some() && self.0[y].len() > x + pos.x {
                        let _ = std::mem::replace(&mut self.0[y + pos.y][x + pos.x], *rune);
                    }
                }
            }
        }
    }

    pub fn width(&self) -> usize {
        self.0.first().unwrap().len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn size(&self) -> Size {
        (self.width(), self.height()).into()
    }
    pub fn fill(&mut self, rect: Rect, rune: Rune) {
        for y in rect.pos.y..(rect.size.height + rect.pos.y).min(self.0.len()) {
            for x in rect.pos.x..(rect.size.width + rect.pos.x).min(self.0[y].len()) {
                let _ = std::mem::replace(&mut self.0[y][x], rune);
            }
        }
    }

    pub fn insert<P: Into<Pos>, S: Into<Runes>>(&mut self, pos: P, value: S) {
        let Pos { x, y } = pos.into();
        let runes: Runes = value.into();
        if let Some(line) = self.0.get_mut(y) {
            let line_len = line.len() as i32;
            for (i, c) in runes
                .iter()
                .take((line_len - x as i32).max(0) as usize)
                .enumerate()
            {
                let _ = std::mem::replace(&mut line[x + i], *c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::Rect, runes::Rune};

    use super::View;

    #[test]
    pub fn test_insert_pos() {
        let mut view = View::new((5, 3));
        view.insert((1, 2), "test");
        dbg!(&view.0);
        assert_eq!(view.0[2][1].content, Some('t'));
        assert_eq!(view.0[2][2].content, Some('e'));
        assert_eq!(view.0[2][3].content, Some('s'));
        assert_eq!(view.0[2][4].content, Some('t'));
    }

    #[test]
    pub fn test_fill() {
        let mut view = View::new((3, 3));
        view.fill(Rect::new((1, 1), (2, 2)), Rune::new().content('X'));
        dbg!(&view.0);
        assert_eq!(view.0[0][0].content, None);
        assert_eq!(view.0[0][1].content, None);
        assert_eq!(view.0[0][2].content, None);

        assert_eq!(view.0[1][0].content, None);
        assert_eq!(view.0[1][1].content, Some('X'));
        assert_eq!(view.0[1][2].content, Some('X'));

        assert_eq!(view.0[2][0].content, None);
        assert_eq!(view.0[2][1].content, Some('X'));
        assert_eq!(view.0[2][2].content, Some('X'));
    }

    #[test]
    pub fn test_fill_overflow() {
        let mut view = View::new((3, 3));
        view.fill(Rect::new((1, 1), (4, 4)), Rune::new().content('X'));
        dbg!(&view.0);
        assert_eq!(view.0[0][0].content, None);
        assert_eq!(view.0[0][1].content, None);
        assert_eq!(view.0[0][2].content, None);

        assert_eq!(view.0[1][0].content, None);
        assert_eq!(view.0[1][1].content, Some('X'));
        assert_eq!(view.0[1][2].content, Some('X'));

        assert_eq!(view.0[2][0].content, None);
        assert_eq!(view.0[2][1].content, Some('X'));
        assert_eq!(view.0[2][2].content, Some('X'));
    }

    #[test]
    pub fn test_apply() {
        let mut view1 = View::new((3, 4));
        view1.fill(Rect::new((1, 1), (2, 2)), Rune::new().content('X'));
        let mut view2 = View::new((3, 4));
        view2.apply((0, 1), view1);
        dbg!(&view2.0);
        assert_eq!(view2.0[0][0].content, None);
        assert_eq!(view2.0[0][1].content, None);
        assert_eq!(view2.0[0][2].content, None);

        assert_eq!(view2.0[1][0].content, None);
        assert_eq!(view2.0[1][1].content, None);
        assert_eq!(view2.0[1][2].content, None);

        assert_eq!(view2.0[2][0].content, None);
        assert_eq!(view2.0[2][1].content, Some('X'));
        assert_eq!(view2.0[2][2].content, Some('X'));

        assert_eq!(view2.0[3][0].content, None);
        assert_eq!(view2.0[3][1].content, Some('X'));
        assert_eq!(view2.0[3][2].content, Some('X'));
    }

    #[test]
    pub fn test_apply_overflow() {
        let mut view0 = View::new((5, 5));
        view0.fill(Rect::new((1, 1), (4, 4)), Rune::new().content('X'));
        let mut view = View::new((3, 3));
        view.apply((0, 0), view0);
        dbg!(&view.0);
        assert_eq!(view.0[0][0].content, None);
        assert_eq!(view.0[0][1].content, None);
        assert_eq!(view.0[0][2].content, None);

        assert_eq!(view.0[1][0].content, None);
        assert_eq!(view.0[1][1].content, Some('X'));
        assert_eq!(view.0[1][2].content, Some('X'));

        assert_eq!(view.0[2][0].content, None);
        assert_eq!(view.0[2][1].content, Some('X'));
        assert_eq!(view.0[2][2].content, Some('X'));
    }
}
