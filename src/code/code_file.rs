use std::{fs::File, hash::DefaultHasher, io::BufReader, rc::Rc};

use log::error;

use super::programming::{ProgrammingLanguage, ProgrammingLineType};

#[derive(Debug)]
pub struct CodeFile<'pf> {
    pub file_path: &'pf str,
    pub lang: &'pf ProgrammingLanguage<'pf>,
    pub blocks: Vec<CodeBlock>,
    pub lines: Vec<CodeLine<5>>,
}

impl<'pf> CodeFile<'pf> {
    pub fn create(file_path: &'pf str, lang: &'pf ProgrammingLanguage) -> Self {
        return CodeFile {
            file_path,
            blocks: Vec::new(),
            lines: Vec::new(),
            lang,
        }
        .generate();
    }

    fn generate(mut self) -> Self {
        let file_result = File::open(self.file_path);

        let file = match file_result {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open file: {}, error: {}", self.file_path, e);
                return self;
            }
        };

        let file_buf_reader = BufReader::new(file);

        let mut hasher = DefaultHasher::new();

        return self;
    }
}

#[derive(Debug)]
struct CodeBlock {
    pub hash: u64,
    block: String,
    code_line: CodeLine<1>,
    block_type: BlockType,
}

#[derive(Debug)]
struct CodeLine<const CHUNK_COUNT: usize> {
    // TODO: This functonality does not exit yet.
    // INFO: The hash will be used for caching. Will store the grammer result in a file with the
    //       hash. When the line hash is the same as the hash in the file us the file grammer
    //       rather then hitting the language API.
    pub hash: u64,
    pub line_number: usize,
    pub original_line: String,
    pub string_line: [Option<CodeChunk>; CHUNK_COUNT],
}

#[derive(Debug)]
struct CodeChunk {
    chunk: Rc<str>,
    chunk_type: ChunkType,
}

#[derive(Debug)]
enum ChunkType {
    Code,
    String,
    Comment,
}

#[derive(Debug)]
enum BlockType {
    String,
    Comment,
}
