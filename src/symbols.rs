#![allow(dead_code)]

#[cfg(any(not(windows), all(windows)))]
mod universal {
    pub const TICK: char = '✔';
    pub const CROSS: char = '✖';
    pub const STAR: char = '★';
    pub const SQUARE: char = '▇';
    pub const SQUARE_SMALL: char = '◻';
    pub const SQUARE_SMALL_FILLED: char = '◼';
    pub const PLAY: char = '▶';
    pub const CIRCLE: char = '◯';
    pub const CIRCLE_FILLED: char = '◉';
    pub const CIRCLE_DOTTED: char = '◌';
    pub const CIRCLE_DOUBLE: char = '◎';
    pub const CIRCLE_CIRCLE: char = 'ⓞ';
    pub const CIRCLE_CROSS: char = 'ⓧ';
    pub const CIRCLE_PIPE: char = 'Ⓘ';
    pub const CIRCLE_QUESTION_MARK: char = '?';
    pub const BULLET: char = '●';
    pub const DOT: char = '․';
    pub const LINE: char = '─';
    pub const ELLIPSIS: char = '…';
    pub const POINTER: char = '❯';
    pub const POINTER_SMALL: char = '›';
    pub const INFO: char = 'ℹ';
    pub const WARNING: char = '⚠';
    pub const HAMBURGER: char = '☰';
    pub const SMILEY: char = '㋡';
    pub const MUSTACHE: char = '෴';
    pub const HEART: char = '♥';
    pub const NODEJS: char = '⬢';
    pub const ARROW_UP: char = '↑';
    pub const ARROW_DOWN: char = '↓';
    pub const ARROW_LEFT: char = '←';
    pub const ARROW_RIGHT: char = '→';
    pub const RADIO_ON: char = '◉';
    pub const RADIO_OFF: char = '◯';
    pub const CHECKBOX_ON: char = '☒';
    pub const CHECKBOX_OFF: char = '☐';
    pub const CHECKBOX_CIRCLE_ON: char = 'ⓧ';
    pub const CHECKBOX_CIRCLE_OFF: char = 'Ⓘ';
    pub const QUESTION_MARK_PREFIX: char = '?';
    pub const ONE_HALF: char = '½';
    pub const ONE_THIRD: char = '⅓';
    pub const ONE_QUARTER: char = '¼';
    pub const ONE_FIFTH: char = '⅕';
    pub const ONE_SIXTH: char = '⅙';
    pub const ONE_SEVENTH: char = '⅐';
    pub const ONE_EIGHTH: char = '⅛';
    pub const ONE_NINTH: char = '⅑';
    pub const ONE_TENTH: char = '⅒';
    pub const TWO_THIRDS: char = '⅔';
    pub const TWO_FIFTHS: char = '⅖';
    pub const THREE_QUARTERS: char = '¾';
    pub const THREE_FIFTHS: char = '⅗';
    pub const THREE_EIGHTHS: char = '⅜';
    pub const FOUR_FIFTHS: char = '⅘';
    pub const FIVE_SIXTHS: char = '⅚';
    pub const FIVE_EIGHTHS: char = '⅝';
    pub const SEVEN_EIGHTHS: char = '⅞';
}

#[cfg(any(not(windows), all(windows)))]
pub use universal::*;

#[cfg(all(windows))]
mod win {
    pub const TICK: char = '√';
    pub const CROSS: char = '×';
    pub const STAR: char = '*';
    pub const SQUARE: char = '█';
    pub const PLAY: char = '►';
    pub const BULLET: char = '*';
    pub const DOT: char = '.';
    pub const LINE: char = '─';
    pub const POINTER: char = '>';
    pub const POINTER_SMALL: char = '»';
    pub const INFO: char = 'i';
    pub const WARNING: char = '‼';
    pub const HAMBURGER: char = '≡';
    pub const SMILEY: char = '☺';
    pub const HEART: char = '♥';
    pub const NODEJS: char = '♦';
    pub const ARROW_UP: char = '↑';
    pub const ARROW_DOWN: char = '↓';
    pub const ARROW_LEFT: char = '←';
    pub const ARROW_RIGHT: char = '→';
    pub const QUESTION_MARK_PREFIX: char = '？';
    pub const ONE_HALF: char = ' ';
}

#[cfg(all(windows))]
pub use win::*;
