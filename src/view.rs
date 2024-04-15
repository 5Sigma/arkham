use crossterm::style::Color;

use crate::{
    geometry::{Pos, Rect, Size},
    runes::{Rune, Runes},
};

/// A renderable region. View stores the renderable state of an area of the
/// screen. Views can be combined together to achieve a finalized view that
/// repsresents the entire screens next render.
#[derive(Clone, Debug)]
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
    /// Construct a new view for a given region size.
    pub fn new<T>(size: T) -> Self
    where
        T: Into<Size>,
    {
        let size: Size = size.into();
        Self(vec![vec![Rune::default(); size.width]; size.height])
    }

    /// Return an iterator for all runes in the view.
    pub fn iter(&self) -> impl Iterator<Item = &Vec<Rune>> {
        self.0.iter()
    }

    /// Apply another view onto this view at a given position.
    pub fn apply<P: Into<Pos>>(&mut self, pos: P, view: &View) {
        let pos = pos.into();
        for (y, line) in view.0.iter().enumerate() {
            if self.0.len() > y + pos.y {
                for (x, rune) in line.iter().enumerate() {
                    if rune.content.is_some() && self.0[y].len() > x + pos.x {
                        let rune = (self.0[y + pos.y][x + pos.x]) + *rune;
                        let _ = std::mem::replace(&mut self.0[y + pos.y][x + pos.x], rune);
                    }
                }
            }
        }
    }

    // The width of the view.
    pub fn width(&self) -> usize {
        self.0.first().map(|i| i.len()).unwrap_or_default()
    }

    /// The height of the view.
    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// The region size of the view.
    pub fn size(&self) -> Size {
        (self.width(), self.height()).into()
    }

    /// Paint is a conveinence method for filling a region ith a given color.
    /// This is done by using the passed color as the background color and
    /// filling the region with ' ' characters.
    pub fn paint<R>(&mut self, rect: R, color: Color)
    where
        R: Into<Rect>,
    {
        self.fill(rect, Rune::new().content(' ').bg(color));
    }

    /// Fill a region of the view with a single rune, repeating it in every
    /// position.
    pub fn fill<R, U>(&mut self, rect: R, rune: U)
    where
        R: Into<Rect>,
        U: Into<Rune>,
    {
        let rect = rect.into();
        let rune = rune.into();
        for y in rect.pos.y..(rect.size.height + rect.pos.y).min(self.0.len()) {
            for x in rect.pos.x..(rect.size.width + rect.pos.x).min(self.0[y].len()) {
                let _ = std::mem::replace(&mut self.0[y][x], rune);
            }
        }
    }

    /// Fill the entire view context with a rune
    pub fn fill_all<R>(&mut self, rune: R)
    where
        R: Into<Rune>,
    {
        let rune = rune.into();
        let rect = Rect::new((0, 0), self.size());
        for y in rect.pos.y..(rect.size.height + rect.pos.y).min(self.0.len()) {
            for x in rect.pos.x..(rect.size.width + rect.pos.x).min(self.0[y].len()) {
                let _ = std::mem::replace(&mut self.0[y][x], rune);
            }
        }
    }

    /// Insert a string at the specific position in the view. Each chacter is
    /// mapped to a rune and placed starting at the position given and
    /// continueing to the right
    ///
    /// This function performs no wrapping of any kind.
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
                let rune = line[x + i] + *c;
                let _ = std::mem::replace(&mut line[x + i], rune);
            }
        }
    }

    #[cfg(test)]
    pub fn render_text(&self) -> String {
        self.0.iter().fold(String::new(), |mut acc, line| {
            acc.push_str(
                &line
                    .into_iter()
                    .map(|r| r.content.unwrap_or_default())
                    .collect::<String>(),
            );
            acc.push('\n');
            acc
        })
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
        view2.apply((0, 1), &view1);
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
        view.apply((0, 0), &view0);
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
