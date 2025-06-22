use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
    rc::Rc,
    sync::Arc,
};

use log::{error, warn};
use tokio::{runtime::Runtime, task::JoinHandle};

use crate::common::string::{DelimiterType, StringDelimiterSlice};

use super::programming::{CodeBlockLineSyntax, ProgrammingLanguage, ProgrammingLineType};

#[derive(Debug)]
pub struct CodeFile<'pf> {
    pub file_path: &'pf str,
    pub lang: &'pf ProgrammingLanguage<'pf>,
    pub blocks: Vec<CodeBlock>,
    pub lines: Vec<CodeLine<5>>,
}

impl<'pf> CodeFile<'pf> {
    pub async fn create(file_path: &'pf str, lang: &'pf ProgrammingLanguage<'pf>) -> Self {
        return CodeFile {
            file_path,
            blocks: Vec::new(),
            lines: Vec::new(),
            lang,
        }
        .generate()
        .await;
    }

    async fn generate(mut self) -> Self {
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
        // let mut raw_lines = Vec::<String>::new();

        let mut line_handles: Vec<JoinHandle<CodeLine<5>>> = Vec::new();
        let mut code_block: Option<CodeBlock> = None;

        for (index, line_res) in file_buf_reader.lines().enumerate() {
            let line = match line_res {
                Ok(line) => line,
                Err(e) => {
                    warn!("Unable to read file line. {}", e);
                    continue;
                }
            };

            let line_number = index + 1;
            line.hash(&mut hasher);

            if matches!(code_block, None) {
                // INFO: Ignore new line in code file
                if line.is_empty() {
                    continue;
                }

                code_block = match self.lang.is_start_of_code_block(&line) {
                    super::programming::CodeBlockType::String(code_block_current_line_syntax) => {
                        Some(CodeBlock::new(
                            line_number,
                            hasher.finish(),
                            line,
                            BlockType::String,
                            code_block_current_line_syntax,
                        ))
                    }
                    super::programming::CodeBlockType::Comment(code_block_syntax) => {
                        Some(CodeBlock::new(
                            line_number,
                            hasher.finish(),
                            line,
                            BlockType::Comment,
                            code_block_syntax,
                        ))
                    }
                    super::programming::CodeBlockType::None => {
                        let hash = hasher.finish();
                        line_handles.push(tokio::task::spawn(CodeLine::new(
                            hash,
                            line_number,
                            line,
                        )));
                        code_block
                    }
                };

                continue;
            }
            // TODO: Handle the unwrap
            code_block = code_block.unwrap().push(line_number, line, &mut hasher);
        }

        for line_handle in line_handles {
            match line_handle.await {
                Ok(code_line) => self.lines.push(code_line),
                Err(e) => {
                    // TODO: Log error
                    error!("Unable to run line concurrently, Error: {:#?}", e);
                }
            }
        }

        return self;
    }

    // TODO:
    fn init_code_block(line: String) {}
}

#[derive(Debug)]
struct CodeBlock {
    pub hash: u64,
    block: String,
    code_line: Vec<CodeLine<1>>,
    block_type: BlockType,
    code_block_current_line_syntax: CodeBlockLineSyntax,
}

impl CodeBlock {
    pub fn new(
        line_number: usize,
        hash: u64,
        line: String,
        block_type: BlockType,
        code_block_current_line_syntax: CodeBlockLineSyntax,
    ) -> Self {
        let line_slices: [Option<&str>; 1] = line.slices_by(
            &code_block_current_line_syntax.start_delimiter,
            &[DelimiterType::None; 0],
        );

        let line_split = match code_block_current_line_syntax.start_delimiter {
            DelimiterType::DelimiterStr(s) => line.split_once(s),
            DelimiterType::DelimiterChar(c) => line.split_once(c),
            DelimiterType::None => None,
        };

        let chunk: Arc<str> = match line_split {
            Some((_, right)) => right.into(),
            None => {
                error!("Error in CodeBlock::New, unable to split line {}", line);
                "".into()
            }
        };

        let block = line.clone();

        let code_chunk = CodeChunk {
            chunk,
            chunk_type: block_type.to_chunk_type(),
        };

        let code_line = CodeLine {
            hash,
            line_number,
            original_line: line,
            chunks: Some([Some(code_chunk)]),
        };

        return Self {
            hash: 0,
            block,
            code_line: vec![code_line],
            block_type,
            code_block_current_line_syntax,
        };
    }

    pub fn push(
        mut self,
        line_number: usize,
        line: String,
        hasher: &mut DefaultHasher,
    ) -> Option<CodeBlock> {
        self.block.push_str(&line);

        let is_end = self.is_end(&line);

        self.push_line(hasher.finish(), line_number, line);

        if is_end {
            self.block.hash(hasher);
            self.hash = hasher.finish();
            // TODO: On the block(self.block) remove the start and end block
            return None;
        }

        return Some(self);
    }

    fn is_end(&self, line: &str) -> bool {
        return match self.code_block_current_line_syntax.end_delimiter {
            DelimiterType::DelimiterStr(s) => !matches!(line.find(s), None),
            DelimiterType::DelimiterChar(c) => !matches!(line.find(c), None),
            DelimiterType::None => true,
        };
    }

    fn push_line(&mut self, hash: u64, line_number: usize, line: String) {
        let code_chunk = CodeChunk {
            chunk: line.as_str().into(),
            chunk_type: self.block_type.to_chunk_type(),
        };

        let code_line = CodeLine {
            hash,
            line_number,
            original_line: line,
            chunks: Some([Some(code_chunk)]),
        };
    }
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
    chunks: Option<[Option<CodeChunk>; CHUNK_COUNT]>,
}

impl<const CHUNK_COUNT: usize> CodeLine<CHUNK_COUNT> {
    pub async fn new(hash: u64, line_number: usize, line: String) -> Self {
        return Self {
            hash,
            line_number,
            original_line: line,
            chunks: None,
        };
    }

    pub fn is_new_line(&self) -> bool {
        return matches!(self.chunks, None);
    }
}

#[derive(Debug)]
struct CodeChunk {
    chunk: Arc<str>,
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

impl BlockType {
    fn to_chunk_type(&self) -> ChunkType {
        return match self {
            BlockType::String => ChunkType::String,
            BlockType::Comment => ChunkType::Comment,
        };
    }
}
