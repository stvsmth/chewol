use crate::FileType;
use crate::Position;
use crate::Row;
use crate::SearchDirection;

use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    dirty: bool,
    file_type: FileType,
    pub filename: Option<String>,
}

impl Document {
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y == len {
            return;
        }
        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
        self.highlight(None);
        self.dirty = true;
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut position = Position { x: at.x, y: at.y };
        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }
        None
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        let mut start_with_comments = false;
        for row in &mut self.rows {
            start_with_comments = row.highlight(
                self.file_type.highlight_options(),
                word,
                start_with_comments,
            );
        }
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
        self.highlight(None);
    }

    pub fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let file_type = FileType::from(filename);
        let mut start_with_comments = false;
        let mut rows = Vec::new();
        for line in contents.lines() {
            let mut row = Row::from(line);
            start_with_comments =
                row.highlight(file_type.highlight_options(), None, start_with_comments);
            rows.push(row);
        }
        Ok(Self {
            dirty: false,
            file_type,
            filename: Some(filename.to_string()),
            rows,
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = fs::File::create(filename)?;
            self.file_type = FileType::from(filename);
            let mut start_with_comment = false;
            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
                start_with_comment = row.highlight(
                    &self.file_type.highlight_options(),
                    None,
                    start_with_comment,
                )
            }
            self.dirty = false;
        }
        Ok(())
    }
}
