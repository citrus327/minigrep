use std::fs;

use crate::{search_param::SearchParam, search_result::SearchResult, utils};

#[derive(Debug)]
pub struct Engine<'a> {
    config: &'a SearchParam,
    results: Vec<SearchResult>,
}

impl<'a> Engine<'a> {
    pub fn from(search_param: &'a SearchParam) -> Self {
        Self {
            config: search_param,
            results: Vec::new(),
        }
    }
    fn read(&self, file_path: &str) -> String {
        let content = fs::read_to_string(file_path);
        let content = content.unwrap_or_else(|e| utils::die(&e));
        content
    }
    pub fn search(&mut self) {
        let content = self.read(&self.config.file_path);

        let mut i = 0;
        for line in content.lines() {
            if line.contains(&self.config.search_string.as_str()) {
                let result: SearchResult = SearchResult::from(i, line.to_string());
                self.results.push(result)
            }

            i += 1;
        }
    }
}
