use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
    rc::Rc,
    sync::Arc,
};

use log::{error, info, warn};
use tokio::{runtime::Runtime, task::JoinHandle};

use crate::common::string::{DelimiterType, StringDelimiterSlice};

use super::programming::{CodeBlockLineSyntax, ProgrammingLanguage, ProgrammingLineType};

#[derive(Debug)]
pub struct CodeFile<'pf, const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize> {
    pub file_path: &'pf str,
    pub lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
    pub blocks: Vec<CodeBlock>,
    pub lines: Vec<Code>,
}

impl<'pf, const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>
    CodeFile<'pf, OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>
{
    pub async fn create(
        file_path: &'pf str,
        lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
    ) -> Self {
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
        let mut line_handles: Vec<JoinHandle<Vec<Code>>> = Vec::new();
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
                        line_handles.push(tokio::task::spawn(Code::generate(
                            hash,
                            line_number,
                            line,
                            self.lang,
                        )));
                        code_block
                    }
                };

                continue;
            }

            if let Some(cb) = code_block {
                let (current_code_block, push_code_block) = cb.push(line_number, line, &mut hasher);
                code_block = current_code_block;

                if let Some(push_code_block) = push_code_block {
                    self.blocks.push(push_code_block);
                }
            }
        }

        for line_handle in line_handles {
            match line_handle.await {
                Ok(codes) => self.lines.extend(codes),
                Err(e) => {
                    // TODO: Log error
                    error!("Unable to run line concurrently, Error: {:#?}", e);
                }
            }
        }

        return self;
    }
}

#[derive(Debug)]
pub struct CodeBlock {
    pub hash: u64,
    block: String,
    code_line: Vec<CodeLine>,
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
        let code_line = CodeLine {
            hash,
            line_number,
            original_line: line,
        };

        let line_split = match code_block_current_line_syntax.start_delimiter {
            DelimiterType::DelimiterStr(s) => code_line.original_line.split_once(s),
            DelimiterType::DelimiterChar(c) => code_line.original_line.split_once(c),
            DelimiterType::None => None,
        };

        let block = match line_split {
            Some((_, right)) => right,
            None => {
                error!(
                    "Error in CodeBlock::New, unable to split line {}",
                    code_line.original_line
                );
                "".into()
            }
        };

        let block = block.to_string();

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
    ) -> (Option<CodeBlock>, Option<CodeBlock>) {
        self.block.push_str(&line);

        // TODO: If it is end line split away from the end block chunk
        let is_end = self.is_end(&line);

        self.push_line(hasher.finish(), line_number, line);

        if is_end {
            self.block.hash(hasher);
            self.hash = hasher.finish();
            return (None, Some(self));
        }

        return (Some(self), None);
    }

    fn is_end(&self, line: &str) -> bool {
        return match self.code_block_current_line_syntax.end_delimiter {
            DelimiterType::DelimiterStr(s) => !matches!(line.find(s), None),
            DelimiterType::DelimiterChar(c) => !matches!(line.find(c), None),
            DelimiterType::None => true,
        };
    }

    fn push_line(&mut self, hash: u64, line_number: usize, line: String) {
        let code_line = CodeLine {
            hash,
            line_number,
            original_line: line,
        };

        self.code_line.push(code_line);
    }
}

#[derive(Debug)]
pub struct Code {
    pub hash: u64,
    pub value: String,
    pub line: CodeLine,
    pub tp: CodeType,
}

impl Code {
    // TODO: Should not use Vec<Code>, for know it is simple
    async fn generate<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
        hash: u64,
        line_number: usize,
        mut line: String,
        lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
    ) -> Vec<Code> {
        let code_line = CodeLine::new(hash, line_number, line.clone());

        // BUG: This code will caus a bug
        // let n = '"'; let t = "soemthing value";
        let mut codes = Vec::<Code>::new();
        loop {
            let (new_line, code) = Code::new_comment_or_string(hash, code_line.clone(), line, lang);

            line = new_line;
            match code {
                Some(code) => codes.push(code),
                None => break,
            }

            // match code {
            //     Some(code) => {
            //         if let CodeType::String = code.tp {
            //             codes.push(code);
            //             break;
            //         }
            //         codes.push(code);
            //     }
            //     None => break,
            // }
        }

        return codes;
    }

    // TODO: Find better name
    fn new_comment_or_string<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
        hash: u64,
        code_line: CodeLine,
        mut line: String,
        lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
    ) -> (String, Option<Code>) {
        let comment_delimiter = lang.comment_delimiter;
        let string_syntax_1 = &lang.string_syntax[0];
        let string_syntax_2 = &lang.string_syntax[1];

        let comment_indexof = line.find(lang.comment_delimiter).unwrap_or(usize::MAX);
        let string_indexof_1 = string_syntax_1
            .string_delimiter
            .indexof(&line)
            .unwrap_or(usize::MAX);
        let string_indexof_2 = string_syntax_2
            .string_delimiter
            .indexof(&line)
            .unwrap_or(usize::MAX);

        if comment_indexof < string_indexof_1 && comment_indexof < string_indexof_2 {
            // TODO: Code in this if should be in its own function
            let comment_split = line.split_once(comment_delimiter);

            line = match comment_split {
                Some((left, right)) => {
                    // TODO: Need to set hash
                    return (
                        left.to_owned(),
                        Some(Code {
                            hash,
                            value: right.trim().to_owned(),
                            line: code_line,
                            tp: CodeType::Comment,
                        }),
                    );
                }
                None => line,
            };
        }

        if string_indexof_1 < comment_indexof && string_indexof_1 < string_indexof_2 {
            // TODO: Code in this if should be in its own function
            let string_slices: [Option<&str>; 1] = line.slices_by(
                &string_syntax_1.string_delimiter,
                &string_syntax_1.string_ignore_delimiter,
            );
            line = match string_slices[0] {
                Some(mut value) => {
                    value = value.trim();

                    if value.is_empty() {
                        return (line, None);
                    }

                    if value.len() <= 3 {
                        return (line.replace(value, ""), None);
                    }

                    return (
                        line.replace(value, ""),
                        Some(Code {
                            hash,
                            value: value.to_owned(),
                            line: code_line,
                            tp: CodeType::String,
                        }),
                    );
                }
                None => line,
            };
        }

        return (line, None);
    }
}

#[derive(Debug, Clone)]
pub struct CodeLine {
    // TODO: This functonality does not exit yet.
    // INFO: The hash will be used for caching. Will store the grammer result in a file with the
    //       hash. When the line hash is the same as the hash in the file us the file grammer
    //       rather then hitting the language API.
    pub hash: u64,
    pub line_number: usize,
    pub original_line: String,
}

impl CodeLine {
    pub fn new(hash: u64, line_number: usize, line: String) -> Self {
        return Self {
            hash,
            line_number,
            original_line: line,
        };
    }
}

#[derive(Debug)]
enum BlockType {
    String,
    Comment,
}

#[derive(Debug)]
pub enum CodeType {
    Code,
    Comment,
    String,
}
