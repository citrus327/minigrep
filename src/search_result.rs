#[derive(Debug)]
pub struct SearchResult {
    pub line_num: i8,
    pub content: String,
}

impl SearchResult {
    pub fn from(line_num: i8, content: String) -> Self {
        Self { line_num, content }
    }

    pub fn print() {}
}
