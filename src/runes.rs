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

impl From<Color> for Rune {
    fn from(value: Color) -> Self {
        Rune {
            content: Some(' '),
            bg: Some(value),
            ..Default::default()
        }
    }
}

impl Rune {
    /// Create a new empty Rune. This can be used with the settings functions as a _builder_ pattern
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let rune:Rune = Rune::new().bg(Color::Blue).fg(Color::White).bold();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content of the rune. The rune's content is a single character.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let rune = Rune::new().content('A');
    /// assert_eq!(rune.content, Some('A'));
    /// ```
    pub fn content(mut self, content: char) -> Self {
        self.content = Some(content);
        self
    }

    /// Set the background color of the rune.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let rune = Rune::new().bg(Color::Green);
    /// assert_eq!(rune.bg, Some(Color::Green));
    /// ```
    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Set the text color of the rune.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let rune = Rune::new().fg(Color::Green);
    /// assert_eq!(rune.fg, Some(Color::Green));
    /// ```
    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    /// Set the text color of the rune.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let rune = Rune::new().fg(Color::Green);
    /// assert_eq!(rune.fg, Some(Color::Green));
    /// ```
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Renders a Print command into the terminal's output queue
    pub(crate) fn render<W>(self, out: &mut W) -> anyhow::Result<()>
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

/// Runes represents a series of runes. This is generally used to convert
/// strings into Runes and apply styling information to them.
///
/// Building runes from a string:
///
/// ```
/// use arkham::prelude::*;
/// let runes = "This is a test string".to_runes().fg(Color::White);
/// ```
#[derive(Clone, Debug, Default)]
pub struct Runes(pub(crate) Vec<Rune>);

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
    /// Create a new runes collection from a vector of Rune.
    pub fn new(runes: Vec<Rune>) -> Self {
        Self(runes)
    }
    /// Set the text color of the rune.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let runes = "blue".to_runes().fg(Color::Blue);
    /// assert!(runes.iter().all(|r| r.fg == Some(Color::Blue)))
    /// ```
    pub fn fg(mut self, color: Color) -> Self {
        for r in self.0.iter_mut() {
            r.fg = Some(color);
        }
        self
    }

    /// Set the text color of the rune.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let mut runes = "on blue".to_runes().bg(Color::Blue);
    /// let runes = runes.clear_fg();
    /// assert!(runes.iter().all(|r| r.fg == None))
    pub fn clear_fg(mut self) -> Self {
        for r in self.0.iter_mut() {
            r.fg = None;
        }
        self
    }

    /// Set the text color of the rune.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let runes = "on blue".to_runes().bg(Color::Blue);
    /// assert!(runes.iter().all(|r| r.bg == Some(Color::Blue)))
    /// ```
    pub fn bg(mut self, color: Color) -> Self {
        for r in self.0.iter_mut() {
            r.bg = Some(color);
        }
        self
    }

    /// Set the text color of the rune.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let mut runes = "on blue".to_runes().bg(Color::Blue);
    /// let runes = runes.clear_bg();
    /// assert!(runes.iter().all(|r| r.bg == None))
    pub fn clear_bg(mut self) -> Self {
        for r in self.0.iter_mut() {
            r.bg = None;
        }
        self
    }

    pub fn bold(mut self) -> Self {
        for r in self.0.iter_mut() {
            r.bold = true;
        }
        self
    }

    /// Append runes or a string displayable object to the Runes
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// let mut runes = "This is a test string. ".to_runes();
    /// runes.add("This is a colored string".to_runes().fg(Color::Blue));
    /// runes.add("This is another basic string");
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
