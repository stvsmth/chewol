#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::return_self_not_must_use,
    clippy::struct_excessive_bools
)]

mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
pub use editor::{Editor, Position, SearchDirection};
pub use filetype::{FileType, HighlightOptions};
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
