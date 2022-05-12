#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::return_self_not_must_use
)]

mod document;
mod editor;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
pub use editor::{Editor, Position, SearchDirection};
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
