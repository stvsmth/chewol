pub struct FileType {
    hl_opts: HighlightOptions,
    name: String,
}

impl FileType {
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
                hl_opts: HighlightOptions {
                    characters: true,
                    numbers: true,
                    strings: true,
                },
                name: String::from("Rust"),
            };
        }
        Self::default()
    }

    pub fn highlight_options(&self) -> HighlightOptions {
        self.hl_opts
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Default, Copy, Clone)]
pub struct HighlightOptions {
    characters: bool,
    numbers: bool,
    strings: bool,
}

impl HighlightOptions {
    pub fn characters(self) -> bool {
        self.characters
    }

    pub fn numbers(self) -> bool {
        self.numbers
    }

    pub fn strings(self) -> bool {
        self.strings
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            hl_opts: HighlightOptions::default(),
            name: String::from("No filetype"),
        }
    }
}
