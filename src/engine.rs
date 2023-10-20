use std::fs;

use crate::{search_param::SearchParam, search_result::SearchResult, utils};

#[derive(Debug)]
pub struct Engine<'a> {
    search_param: &'a SearchParam,
    results: Vec<SearchResult>,
}

impl<'a> Engine<'a> {
    pub fn from(search_param: &'a SearchParam) -> Self {
        Self {
            search_param: search_param,
            results: Vec::new(),
        }
    }
    fn read(&self, file_path: &str) -> String {
        let content = fs::read_to_string(file_path);
        content.unwrap_or_else(|e| utils::die(&e))
    }

    pub fn search(&mut self) -> &Self {
        let content = self.read(&self.search_param.file_path);

        let mut i = 0;
        for (_, line) in content.lines().enumerate() {
            if line.contains(self.search_param.search_string.as_str()) {
                let result: SearchResult = SearchResult::from(i, line.to_string());
                self.results.push(result)
            }

            i += 1;
        }
        self
    }

    pub fn pretty_print(&self) {
        println!("Found {} occurrences", &self.results.len());
        for element in &self.results {
            println!("{} in line {}", element.content, element.line_num)
        }
    }
}
