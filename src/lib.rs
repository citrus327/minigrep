pub mod engine;
pub mod search_param;
pub mod search_result;
mod utils;

#[cfg(test)]
mod tests {
    use std::fs;

    mod search_param_test {
        use std::env;

        use crate::search_param::SearchParam;

        #[test]
        fn test_from_args() {
            // let test_args = vec!["program_name", "--option", "value"];

            // Set the test arguments for the duration of this test
            // env::args().collect::<Vec<String>>();
            // env::args_mut()
            //     .take(1)
            //     .chain(test_args.iter().map(|s| s.to_string()));

            env::set_var("ARGS", "argument1 argument2");

            let search_param = SearchParam::from_args();

            assert_eq!(search_param.file_path, "123");
            assert_eq!(search_param.search_string, "123");
        }
    }

    #[test]
    fn test_read_existing_file() {
        // Create a temporary file with some content
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write content to the file
        let content = "Lorem Lorem\n Chuck Chuck \n Yuuuuu";
        fs::write(file_path, content).expect("Failed to write to file");
        println!("file: {}", file_path);
        // Call the read function and assert the result
        // let result = read(&file_path);
        // assert_eq!(result, content);
    }
}
