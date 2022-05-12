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
                hl_opts: HighlightOptions { numbers: true },
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
    numbers: bool,
}

impl HighlightOptions {
    pub fn numbers(self) -> bool {
        self.numbers
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
