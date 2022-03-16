use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct FileReader;

impl FileReader {
    /// Initialize a new `FileReader`
    ///
    /// # Example
    ///
    /// ```rust
    /// let file_reader = FileReader::new();
    /// ```
    ///
    /// # Returns
    ///
    /// A new `FileReader` instance
    pub fn new() -> FileReader {
        FileReader {}
    }

    /// Read the lines of a file
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path of the file that should be read
    ///
    /// # Example
    ///
    /// ```rust
    /// let lines: Vec<String> = file_reader.read_lines("/path/to/file");
    /// ```
    ///
    /// # Returns
    ///
    /// A `Vec` that contains all the lines in the specified file
    pub fn read_lines(&self, path: &str) -> Vec<String> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut lines = vec![];
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            lines.push(line);
        }

        lines
    }
}
