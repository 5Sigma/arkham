use std::ops::{Add, AddAssign, Sub};

/// Pos represents a coordinate position within the termianl screen.
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<usize> for Pos {
    fn from(value: usize) -> Self {
        Self { x: value, y: value }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Pos) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// An area that can be operated on.
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Add<Size> for Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub<Size> for Size {
    type Output = Size;

    fn sub(self, rhs: Size) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl Add<i32> for Size {
    type Output = Size;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width + rhs as usize,
            height: self.height + rhs as usize,
        }
    }
}

impl Sub<i32> for Size {
    type Output = Size;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width - rhs as usize,
            height: self.height - rhs as usize,
        }
    }
}

impl From<(usize, usize)> for Size {
    fn from(value: (usize, usize)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

impl From<(u16, u16)> for Size {
    fn from(value: (u16, u16)) -> Self {
        Self {
            width: value.0 as usize,
            height: value.1 as usize,
        }
    }
}

impl From<(i32, i32)> for Size {
    fn from(value: (i32, i32)) -> Self {
        Self {
            width: value.0 as usize,
            height: value.1 as usize,
        }
    }
}

impl From<i32> for Size {
    fn from(value: i32) -> Self {
        Self {
            width: value as usize,
            height: value as usize,
        }
    }
}

/// An area of the screen with a given size and postiion. The position
/// represents the top-left corner of the rectangle.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos: Pos,
    pub size: Size,
}

impl Rect {
    pub fn new<P, S>(pos: P, size: S) -> Self
    where
        P: Into<Pos>,
        S: Into<Size>,
    {
        Self {
            pos: pos.into(),
            size: size.into(),
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn with_size<S>(size: S) -> Self
    where
        S: Into<Size>,
    {
        Rect {
            pos: (0, 0).into(),
            size: size.into(),
        }
    }
}

impl From<Size> for Rect {
    fn from(value: Size) -> Self {
        Rect::with_size(value)
    }
}
