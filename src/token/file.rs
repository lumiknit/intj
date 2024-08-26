// File helper for tokenizer / parser

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub contents: String,
}

pub trait FileLoader {
    fn load_file(&mut self, name: &str) -> Result<File, String>;
}

/// STD file loader
pub struct StdFileLoader {}

impl FileLoader for StdFileLoader {
    fn load_file(&mut self, name: &str) -> Result<File, String> {
        match std::fs::read_to_string(name) {
            Ok(contents) => Ok(File {
                name: name.to_string(),
                contents: contents,
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}

/// Position of a token in a file
#[derive(Debug, Clone)]
pub struct Pos {
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

impl File {
    pub fn find_position(&self, pos: usize) -> Pos {
        let mut line = 0;
        let mut col = 0;
        for (i, c) in self.contents.chars().enumerate() {
            if i == pos {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        Pos { pos, line, col }
    }
}
