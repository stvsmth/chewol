use termion::color;

#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Character,
    Comment,
    Match,
    MultilineComment,
    None,
    PrimaryKeyword,
    Number,
    SecondaryKeyword,
    String,
}

impl Mode {
    pub fn to_color(self) -> impl color::Color {
        match self {
            Mode::Character => color::Rgb(108, 113, 196),
            Mode::Comment | Mode::MultilineComment => color::Rgb(133, 153, 0),
            Mode::Match => color::Rgb(38, 139, 210),
            Mode::PrimaryKeyword => color::Rgb(181, 137, 0),
            Mode::Number => color::Rgb(220, 163, 163),
            Mode::SecondaryKeyword => color::Rgb(42, 161, 152),
            Mode::String => color::Rgb(211, 54, 130),
            Mode::None => color::Rgb(255, 255, 255),
        }
    }
}
