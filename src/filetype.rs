#[derive(Default)]
pub struct HighlightOptions {
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    numbers: bool,
    strings: bool,
    secondary_keywords: Vec<String>,
}

impl HighlightOptions {
    pub fn characters(&self) -> bool {
        self.characters
    }

    pub fn comments(&self) -> bool {
        self.comments
    }

    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }

    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }

    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }

    pub fn numbers(&self) -> bool {
        self.numbers
    }

    pub fn strings(&self) -> bool {
        self.strings
    }
}

pub struct FileType {
    hl_opts: HighlightOptions,
    name: String,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            hl_opts: HighlightOptions::default(),
            name: String::from("No filetype"),
        }
    }
}

impl FileType {
    pub fn highlight_options(&self) -> &HighlightOptions {
        &self.hl_opts
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn from(filename: &str) -> Self {
        // TODO: Make this a utility function
        // do case-insensitive check of filename extension
        if filename
            .rsplit('.')
            .next()
            .map(|ext| ext.eq_ignore_ascii_case("rs"))
            == Some(true)
        {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightOptions {
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    numbers: true,
                    strings: true,
                    primary_keywords: vec![
                        "as".to_string(),
                        "break".to_string(),
                        "const".to_string(),
                        "continue".to_string(),
                        "crate".to_string(),
                        "else".to_string(),
                        "enum".to_string(),
                        "extern".to_string(),
                        "false".to_string(),
                        "fn".to_string(),
                        "for".to_string(),
                        "if".to_string(),
                        "impl".to_string(),
                        "in".to_string(),
                        "let".to_string(),
                        "loop".to_string(),
                        "match".to_string(),
                        "mod".to_string(),
                        "move".to_string(),
                        "mut".to_string(),
                        "pub".to_string(),
                        "ref".to_string(),
                        "return".to_string(),
                        "self".to_string(),
                        "Self".to_string(),
                        "static".to_string(),
                        "struct".to_string(),
                        "super".to_string(),
                        "trait".to_string(),
                        "true".to_string(),
                        "type".to_string(),
                        "unsafe".to_string(),
                        "use".to_string(),
                        "where".to_string(),
                        "while".to_string(),
                        "dyn".to_string(),
                        "abstract".to_string(),
                        "become".to_string(),
                        "box".to_string(),
                        "do".to_string(),
                        "final".to_string(),
                        "macro".to_string(),
                        "override".to_string(),
                        "priv".to_string(),
                        "typeof".to_string(),
                        "unsized".to_string(),
                        "virtual".to_string(),
                        "yield".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "try".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(),
                        "char".to_string(),
                        "i8".to_string(),
                        "i16".to_string(),
                        "i32".to_string(),
                        "i64".to_string(),
                        "isize".to_string(),
                        "u8".to_string(),
                        "u16".to_string(),
                        "u32".to_string(),
                        "u64".to_string(),
                        "usize".to_string(),
                        "f32".to_string(),
                        "f64".to_string(),
                    ],
                },
            };
        }
        Self::default()
    }
}
