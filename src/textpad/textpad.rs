use crate::textfile::{self };

pub struct TextPad {
    pub file_obj: textfile::textfile::TextFile,
}

impl TextPad {
    pub fn new(path: String) -> Self {
        return Self {
            file_obj: textfile::textfile::TextFile::new(path),
        }
    }
}
