use termion::color;

#[derive(PartialEq, Debug)]
pub enum Type {
    Character,
    Comment,
    Match,
    None,
    Number,
    String,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Type::Character => color::Rgb(108, 113, 196),
            Type::Comment => color::Rgb(133, 153, 0),
            Type::Match => color::Rgb(38, 139, 210),
            Type::Number => color::Rgb(220, 163, 163),
            Type::String => color::Rgb(211, 54, 130),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
