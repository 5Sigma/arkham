use crossterm::{
    queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
};

/// Rune repesents the state of the screen at a specific position. It stores
/// the character content and styling information that will be rendered.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Rune {
    pub content: Option<char>,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
}

impl std::fmt::Debug for Rune {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Rn({})", self.content.unwrap_or_default()))
            .finish()
    }
}

impl std::ops::Add<Rune> for Rune {
    type Output = Rune;

    fn add(self, mut rhs: Rune) -> Self::Output {
        rhs.fg = rhs.fg.or(self.fg);
        rhs.bg = rhs.bg.or(self.bg);
        rhs
    }
}

impl From<char> for Rune {
    fn from(value: char) -> Self {
        Rune {
            content: Some(value),
            ..Default::default()
        }
    }
}

impl Rune {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn content(mut self, content: char) -> Self {
        self.content = Some(content);
        self
    }
    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn render<W>(self, out: &mut W) -> anyhow::Result<()>
    where
        W: std::io::Write,
    {
        if let Some(content) = self.content {
            queue!(out, ResetColor)?;
            if let Some(c) = self.fg {
                queue!(out, SetForegroundColor(c))?;
            }
            if let Some(c) = self.bg {
                queue!(out, SetBackgroundColor(c))?;
            }
            if self.bold {
                queue!(out, SetAttribute(Attribute::Bold))?;
            }
            queue!(out, Print(content))?;
        }
        Ok(())
    }
}

/// Runes represetns a series of runes. This is generally used to convert
/// strings into Runes and apply styling information to them.
#[derive(Clone, Debug, Default)]
pub struct Runes(Vec<Rune>);

impl std::ops::Deref for Runes {
    type Target = Vec<Rune>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Rune> for Runes {
    fn from(value: Rune) -> Self {
        Runes::new(vec![value])
    }
}

impl<T: ToString> From<T> for Runes {
    fn from(value: T) -> Self {
        Runes(
            value
                .to_string()
                .chars()
                .map(|c| Rune::new().content(c))
                .collect(),
        )
    }
}

impl Runes {
    pub fn new(runes: Vec<Rune>) -> Self {
        Self(runes)
    }
    pub fn fg(self, color: Color) -> Self {
        self.set_fg(Some(color))
    }

    pub fn set_fg(mut self, color: Option<Color>) -> Self {
        for r in self.0.iter_mut() {
            r.fg = color;
        }
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        for r in self.0.iter_mut() {
            r.bg = Some(color);
        }
        self
    }
    pub fn bold(mut self) -> Self {
        for r in self.0.iter_mut() {
            r.bold = true;
        }
        self
    }

    pub fn add<R>(&mut self, runes: R)
    where
        R: Into<Runes>,
    {
        self.0.append(&mut runes.into().0);
    }
}

pub trait ToRuneExt {
    fn to_runes(&self) -> Runes;
}

impl ToRuneExt for String {
    fn to_runes(&self) -> Runes {
        Runes::from(self.to_string())
    }
}

impl ToRuneExt for &str {
    fn to_runes(&self) -> Runes {
        Runes::from(self.to_string())
    }
}
