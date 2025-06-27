use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
    rc::Rc,
    sync::Arc,
};

use log::{error, info, warn};
use tokio::{runtime::Runtime, task::JoinHandle};

use crate::{
    code::programming::ProgrammingStringSyntax,
    common::string::{DelimiterType, StringDelimiter, StringDelimiterSlice, StringSlice},
    nvim_lang_dictionary::NvimLanguageReadonlyDictionary,
};

use super::programming::{CodeBlockLineSyntax, ProgrammingLanguage};

#[derive(Debug)]
pub struct CodeFile<'pf, const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize> {
    pub file_path: &'pf str,
    pub lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
    pub blocks: Vec<CodeBlock>,
    pub lines: Vec<Code>,
    nvim_language_readonly_dictionary: Arc<NvimLanguageReadonlyDictionary>,
}

impl<'pf, const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>
    CodeFile<'pf, OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>
{
    pub async fn create(
        file_path: &'pf str,
        lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
        nvim_language_readonly_dictionary: NvimLanguageReadonlyDictionary,
    ) -> Self {
        return CodeFile {
            file_path,
            blocks: Vec::new(),
            lines: Vec::new(),
            lang,
            nvim_language_readonly_dictionary: Arc::new(nvim_language_readonly_dictionary),
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

        let nvim_language_readonly_dictionary = self.nvim_language_readonly_dictionary.clone();
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
                            nvim_language_readonly_dictionary.clone(),
                        )));
                        code_block
                    }
                };

                continue;
            }

            if let Some(cb) = code_block {
                let (current_code_block, push_code_block) =
                    cb.push(line_number, line, &mut hasher, self.lang);
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
    pub block: String,
    pub lines: Vec<CodeLine>,
    pub block_type: BlockType,
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

        let mut block = block.to_string();
        block.push('\n');

        return Self {
            hash: 0,
            block,
            lines: vec![code_line],
            block_type,
            code_block_current_line_syntax,
        };
    }

    pub fn push<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
        mut self,
        line_number: usize,
        line: String,
        hasher: &mut DefaultHasher,
        lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
    ) -> (Option<CodeBlock>, Option<CodeBlock>) {
        self.block.push_str(&line);
        self.block.push('\n');

        let is_end = self.is_end(&line);

        self.push_line(hasher.finish(), line_number, line);

        if is_end {
            self.block.hash(hasher);
            self.hash = hasher.finish();
            self.block = self
                .block
                .trim()
                .trim_end_by_delimiter(&self.code_block_current_line_syntax.end_delimiter)
                .trim()
                .trim_end_matches(&lang.comment_delimiter)
                .trim()
                .to_owned();
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

        self.lines.push(code_line);
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
        nvim_language_readonly_dictionary: Arc<NvimLanguageReadonlyDictionary>,
    ) -> Vec<Code> {
        let code_line = CodeLine::new(hash, line_number, line.clone());

        line = lang.post_replace_with_empty_space(line);

        let mut codes = Vec::<Code>::new();
        loop {
            // WARN: This is just for safty
            if codes.len() > 20 {
                error!("Code::generate posible infinit loop. Line: {}", line);
                break;
            }

            let code_line_state = Code::new_comment_or_string(hash, code_line.clone(), line, lang);

            match code_line_state {
                CodeLineState::ContinueWithResult(new_line, code) => {
                    line = new_line;
                    codes.push(code);
                }
                CodeLineState::Continue(new_line) => line = new_line,
                CodeLineState::Done(new_line) => {
                    line = new_line;
                    break;
                }
            }
        }

        line = lang.replase_all_operators_and_syntax_with_whitespace(line);
        line = lang.replase_all_reserved_keywords_with_whitespace(
            line,
            &nvim_language_readonly_dictionary,
        );
        line = line.trim().to_owned();

        if line.is_empty() {
            return codes;
        }

        let code = Code {
            hash,
            value: line,
            line: code_line,
            tp: CodeType::Code,
        };
        codes.push(code);
        return codes;
    }

    // TODO: Find better name
    fn new_comment_or_string<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
        hash: u64,
        code_line: CodeLine,
        mut line: String,
        lang: &'static ProgrammingLanguage<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
    ) -> CodeLineState {
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
        let comment_block_line_syntax = lang.block_comment.get_code_block_line_syntax(&line);
        let string_block_line_syntax = lang.block_string.get_code_block_line_syntax(&line);

        if comment_indexof < string_indexof_1
            && comment_indexof < string_indexof_2
            && comment_indexof < comment_block_line_syntax.start_indexof
            && comment_indexof < string_block_line_syntax.start_indexof
        {
            return Code::new_comment(hash, line, code_line, comment_delimiter);
        }

        if comment_block_line_syntax.start_indexof < string_indexof_1
            && comment_block_line_syntax.start_indexof < string_indexof_2
            && comment_block_line_syntax.start_indexof < string_block_line_syntax.start_indexof
        {
            return Code::new_block(
                hash,
                line,
                code_line,
                CodeType::Comment,
                &comment_block_line_syntax,
            );
        }

        if string_block_line_syntax.start_indexof < string_indexof_1
            && string_block_line_syntax.start_indexof < string_indexof_2
        {
            return Code::new_block(
                hash,
                line,
                code_line,
                CodeType::String,
                &string_block_line_syntax,
            );
        }

        if string_indexof_1 < string_indexof_2 {
            return Code::new_string(hash, line, code_line, string_syntax_1);
        }

        if string_indexof_2 != usize::MAX {
            return Code::new_string(hash, line, code_line, string_syntax_2);
        }

        return CodeLineState::Done(line);
    }

    fn new_comment(
        hash: u64,
        line: String,
        code_line: CodeLine,
        comment_delimiter: &str,
    ) -> CodeLineState {
        let comment_split = line.split_once(comment_delimiter);

        return match comment_split {
            Some((left, right)) => CodeLineState::ContinueWithResult(
                left.to_owned(),
                Code {
                    hash,
                    value: right.trim().to_owned(),
                    line: code_line,
                    tp: CodeType::Comment,
                },
            ),
            None => CodeLineState::Continue(line),
        };
    }

    fn new_string(
        hash: u64,
        line: String,
        code_line: CodeLine,
        string_syntax: &ProgrammingStringSyntax,
    ) -> CodeLineState {
        return Code::new(
            hash,
            line,
            code_line,
            CodeType::String,
            &string_syntax.string_delimiter,
            &string_syntax.string_delimiter,
            &string_syntax.string_ignore_delimiter,
        );
    }

    fn new_block(
        hash: u64,
        line: String,
        code_line: CodeLine,
        code_type: CodeType,
        block_line_syntax: &CodeBlockLineSyntax,
    ) -> CodeLineState {
        return Code::new(
            hash,
            line,
            code_line,
            code_type,
            &block_line_syntax.start_delimiter,
            &block_line_syntax.end_delimiter,
            &[DelimiterType::None, DelimiterType::None],
        );
    }

    fn new(
        hash: u64,
        line: String,
        code_line: CodeLine,
        code_type: CodeType,
        start_delimiter: &DelimiterType,
        end_delimiter: &DelimiterType,
        ignore_by_delimiters: &[DelimiterType; 2],
    ) -> CodeLineState {
        let mut string_slice: Option<&str> = None;

        if start_delimiter == end_delimiter {
            let string_slices: [Option<&str>; 1] =
                line.slices_by(start_delimiter, ignore_by_delimiters);

            string_slice = string_slices[0];
        } else {
            string_slice = line.delimiter_slice_between(start_delimiter, end_delimiter);
        }

        // TODO: Split string by nameing convetion. Ignore strings with code in it.
        return match string_slice {
            Some(mut value) => {
                let mut replace_value = match start_delimiter {
                    DelimiterType::DelimiterStr(s) => format!("{}{}", s, value),
                    DelimiterType::DelimiterChar(c) => format!("{}{}", c, value),
                    DelimiterType::None => String::new(),
                };

                replace_value = match end_delimiter {
                    DelimiterType::DelimiterStr(s) => format!("{}{}", replace_value, s),
                    DelimiterType::DelimiterChar(c) => format!("{}{}", replace_value, c),
                    DelimiterType::None => String::new(),
                };

                value = value.trim();

                if value.is_empty() {
                    return CodeLineState::Continue(line.replace(&replace_value, ""));
                }

                // INFO: This will ignore two char blocks
                if value.len() <= 2 {
                    return CodeLineState::Continue(line.replace(&replace_value, ""));
                }

                return CodeLineState::ContinueWithResult(
                    line.replace(&replace_value, ""),
                    Code {
                        hash,
                        value: value.to_owned(),
                        line: code_line,
                        tp: code_type,
                    },
                );
            }
            None => CodeLineState::Continue(line),
        };
    }
}

#[derive(Debug)]
enum CodeLineState {
    ContinueWithResult(String, Code),
    Continue(String),
    Done(String),
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
