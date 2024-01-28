use std::{borrow::Borrow, collections::HashMap, error::Error};

#[derive(Clone, Default)]
pub struct FileRow {
    pub content: String,
    pub row_number: i32,
}

impl FileRow {
    fn new(row_number: i32, content: String) -> Self {
        return Self {
            content,
            row_number,
        };
    }
}

#[derive(Clone)]
pub struct TextFile {
    pub max_lines: i32,
    pub rowid_vs_row: HashMap<i32, FileRow>,
    pub path: String,
}

impl TextFile {
    pub fn new(path: String) -> Self {
        return Self {
            path,
            max_lines: 100,
            rowid_vs_row: HashMap::<i32, FileRow>::new(),
        };
    }

    pub fn add_content_to_row(&mut self, rowid: i32, content: String) {
        self.rowid_vs_row
            .entry(rowid)
            .or_insert(FileRow::new(rowid, content.clone()))
            .content += &content.borrow();
    }

    pub fn remove_row(&mut self, rowid: i32) {
        self.rowid_vs_row.remove(rowid.borrow());
    }

    pub fn get_row(self, rowid: i32) -> Result<String, &'static str> {
        let r: Option<&FileRow> = self.rowid_vs_row.get(rowid.borrow());
        match r {
            Some(x) => {
                Ok(x.content.clone())
            }

            None => {
                Err("row not found")
            }
        }
    }
}
