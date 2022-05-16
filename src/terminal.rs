use crate::Position;

use std::io::{self, stdout, Write};
use termion::{
    clear, color, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2), // accommodate status bar height
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_current_line() {
        print!("{}", clear::CurrentLine);
    }

    pub fn clear_screen() {
        print!("{}", clear::All);
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(pos: &Position) {
        let Position { mut x, mut y } = pos;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", cursor::Goto(x, y));
    }

    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
