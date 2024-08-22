use std::fs::File;
use std::io::{self, Read};

struct FileReader {
    filename: String,
    content: Option<String>,
}

impl FileReader {
    fn new(file_path: &str) -> FileReader {
        FileReader {
            filename: file_path.to_string(),
            content: None,
        }
    }

    fn read_content(&mut self) -> Result<String, io::Error> {
        let mut file = File::open(&self.filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // self.content = Some(content);
        Ok(content)
    }
}

fn main() {
    let mut f = FileReader::new("file.txt");
    let content = f.read_content();
    match content {
        Ok(c) => println!("Content: {}", c),
        Err(e) => println!("Error: {}", e),
    }
}
