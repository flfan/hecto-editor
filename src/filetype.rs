pub struct FileType {
    name: String,
    hl_ops: HighlightingOptions,
}

#[derive(Default, Clone, Copy)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_ops: HighlightingOptions::default(),
        }
    }
}

impl HighlightingOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }

    pub fn strings(self) -> bool {
        self.strings
    }

    pub fn characters(self) -> bool {
        self.characters
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.hl_ops
    }

    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_ops: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                },
            };
        }
        Self::default()
    }
}
