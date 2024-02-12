use std::{
    char,
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
    ops::Add,
    str::from_utf8_unchecked,
};

use log::{error, info, warn};

use crate::common::UPPER_CASE_ALPHABET;

#[derive(Debug)]
pub enum ProgrammingLanguageType {
    Lua,
    Rust,
}

#[derive(Debug)]
pub enum ProgrammingLineType {
    NewLine,
    Code,
    CodeWithComment,
    Comment,
    BlockCommentStart,
    BlockComment,
    BlockCommentEnd,
    BlockCommentStartAndEnd,
    Unknown,
}

#[derive(Debug)]
pub enum NamingConvetionType {
    CamelCase,
    PascalCase,
    None,
}

#[derive(Debug)]
pub struct ProgrammingLanguage<'lang> {
    pub extension: &'lang str,
    comment_delimiter: &'lang str,
    block_comment_delimiter_start: &'lang str,
    block_comment_delimiter_end: &'lang str,
    operators_and_syntax: Vec<&'lang str>,
    reserved_keywords: Vec<&'lang str>,
    string_syntax: [char; 1],
    naming_convetions: [NamingConvetionType; 2],
    lang_type: ProgrammingLanguageType,
}

impl<'lang> ProgrammingLanguage<'lang> {
    pub fn init() -> [ProgrammingLanguage<'lang>; 2] {
        return [
            ProgrammingLanguage {
                extension: ".lua",
                comment_delimiter: "--",
                block_comment_delimiter_start: "",
                block_comment_delimiter_end: "",
                string_syntax: ['"'],
                reserved_keywords: vec![],
                operators_and_syntax: vec![],
                naming_convetions: [NamingConvetionType::None, NamingConvetionType::None],
                lang_type: ProgrammingLanguageType::Lua,
            },
            ProgrammingLanguage {
                extension: ".rs",
                comment_delimiter: "//",
                block_comment_delimiter_start: "/*",
                block_comment_delimiter_end: "*/",
                string_syntax: ['"'],
                reserved_keywords: vec![
                    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else",
                    "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
                    "match", "mod", "move", "mut", "pub", "ref", "return", "Self", "self",
                    "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
                    "while",
                ],
                operators_and_syntax: vec![
                    "+", "-", "*", "/", "%", "=", "!", ">", "<", "&", "^", "/=", "%=", "(", ")",
                    "{", "}", "[", "]", ";", ":", ",", "..", ".", "#",
                ],
                naming_convetions: [NamingConvetionType::PascalCase, NamingConvetionType::None],
                lang_type: ProgrammingLanguageType::Rust,
            },
        ];
    }

    pub fn is_reserved_keyword(&self, input: &str) -> bool {
        for reserved_keyword in &self.reserved_keywords {
            if input == *reserved_keyword {
                return true;
            }
        }

        return false;
    }

    pub fn replase_all_operators_and_syntax_with_whitespace(&self, input: &str) -> String {
        let mut transform = String::from(input);

        for op_snt in &self.operators_and_syntax {
            transform = transform.replace(op_snt, " ");
        }

        return transform;
    }

    // WARN: This might not work on utf16 strings!
    pub fn split_by_naming_conventions(&self, input: &str) -> String {
        const LOWERCASE_UTF8: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const UPPERCASE_UTF8: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut output = String::new();

        let input_bytes = input.as_bytes();
        let mut current_index: usize = 0;
        let mut start_index: usize = 0;
        let mut is_first_uppercase = true;

        for input_byte in input_bytes {
            for up_alp in UPPERCASE_UTF8 {
                if input_byte == up_alp && is_first_uppercase {
                    start_index = current_index;
                    is_first_uppercase = false;
                    break;
                }

                if input_byte == up_alp {
                    let l = &input_bytes[start_index..current_index];
                    let k = unsafe { from_utf8_unchecked(l) };
                    output.push(' ');
                    output.push_str(k);
                    start_index = current_index;
                    break;
                }
            }

            current_index += 1;
        }

        if start_index < current_index {
            let l = &input_bytes[start_index..current_index];
            let k = unsafe { from_utf8_unchecked(l) };
            output.push(' ');
            output.push_str(k);
        }

        if output.is_empty() {
            return String::from(input);
        }

        return output;
    }
}

#[derive(Debug)]
pub struct ProgrammingFile<'pf> {
    pub file_path: &'pf str,
    pub lang: &'pf ProgrammingLanguage<'pf>,
    pub lines: Vec<ProgrammingLine>,
}

impl<'pf> ProgrammingFile<'pf> {
    pub fn create(file_path: &'pf str, lang: &'pf ProgrammingLanguage) -> Self {
        let mut prog_file = ProgrammingFile {
            file_path,
            lines: Vec::new(),
            lang,
        };

        prog_file.generate_line();

        return prog_file;
    }

    fn generate_line(&mut self) {
        let file_result = File::open(self.file_path);

        let file = match file_result {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open file: {}, error: {}", self.file_path, e);
                return;
            }
        };

        let file_buf_reader = BufReader::new(file);

        let mut hasher = DefaultHasher::new();

        for (index, line_res) in file_buf_reader.lines().enumerate() {
            let line = match line_res {
                Ok(line) => line,
                Err(e) => {
                    warn!("Unable to read file line. {}", e);
                    continue;
                }
            };

            let mut programming_line = ProgrammingLine::new(index + 1, line);
            programming_line.set_type(self.lang, self.lines.last());
            programming_line.set_if_comment(self.lang);
            programming_line.set_if_block_comment();
            programming_line.set_if_code();
            programming_line.set_hash(&mut hasher);

            self.lines.push(programming_line);
        }
    }

    pub fn debug_p(&self) {
        for line in &self.lines {
            info!("{:?}", line.debug_ptrs());
        }
    }
}

// TODO: Find better name
#[derive(Debug)]
pub struct ProgrammingLine {
    pub hash: u64,
    pub line_number: usize,
    pub original_line: String,
    pub commented_line: Option<*const str>,
    pub code_line: Option<*const str>,
    pub prog_type: ProgrammingLineType,
}

impl ProgrammingLine {
    pub fn new(line_number: usize, original_line: String) -> Self {
        return ProgrammingLine {
            hash: 0,
            line_number,
            original_line,
            commented_line: None,
            code_line: None,
            prog_type: ProgrammingLineType::Unknown,
        };
    }

    pub fn set_type(
        &mut self,
        lang: &ProgrammingLanguage,
        last_prog_line_type: Option<&ProgrammingLine>,
    ) {
        let line = &self.original_line;

        if line.is_empty() {
            self.prog_type = ProgrammingLineType::NewLine;
            return;
        }

        if line.contains(lang.comment_delimiter) {
            self.prog_type = ProgrammingLineType::Comment;
            return;
        }

        let is_block_cmt_start = line.contains(lang.block_comment_delimiter_start);
        let is_block_cmt_end = line.contains(lang.block_comment_delimiter_end);

        if is_block_cmt_start && is_block_cmt_end {
            self.prog_type = ProgrammingLineType::BlockCommentStartAndEnd;
            return;
        } else if is_block_cmt_start {
            self.prog_type = ProgrammingLineType::BlockCommentStart;
            return;
        } else if is_block_cmt_end {
            self.prog_type = ProgrammingLineType::BlockCommentEnd;
            return;
        }

        //Check if line is the body of a block comment
        if let Some(prog_line) = last_prog_line_type {
            if matches!(prog_line.prog_type, ProgrammingLineType::BlockComment)
                || matches!(prog_line.prog_type, ProgrammingLineType::BlockCommentStart)
            {
                self.prog_type = ProgrammingLineType::BlockComment;
                return;
            }
        }

        //TODO: Need to use the lang to check it this is really code.
        self.prog_type = ProgrammingLineType::Code;
    }

    fn set_if_comment(&mut self, lang: &ProgrammingLanguage) {
        if !matches!(self.prog_type, ProgrammingLineType::Comment) {
            return;
        }

        let line_comment_option = self.original_line.split_once(lang.comment_delimiter);

        if let Some((left_of_line, right_of_line)) = line_comment_option {
            self.commented_line = Some(right_of_line.trim());

            if left_of_line.trim().is_empty() {
                return;
            }
            // TODO: The code line need to be transformed
            self.code_line = Some(left_of_line);
            self.prog_type = ProgrammingLineType::CodeWithComment;
        }
    }

    // TODO: What if there is code on the same line.
    //       What if there is two or more block comments on the same line.
    fn set_if_block_comment(&mut self) {
        match self.prog_type {
            ProgrammingLineType::BlockCommentStart => (),
            ProgrammingLineType::BlockComment => (),
            ProgrammingLineType::BlockCommentEnd => (),
            ProgrammingLineType::BlockCommentStartAndEnd => (),
            _ => return,
        }

        self.commented_line = Some(self.original_line.trim());
    }

    fn set_if_code(&mut self) {
        if !matches!(self.prog_type, ProgrammingLineType::Code) {
            return;
        }

        // TODO: The code line need to be transformed
        self.code_line = Some(self.original_line.as_str());
    }

    fn set_hash(&mut self, hasher: &mut DefaultHasher) {
        if matches!(self.prog_type, ProgrammingLineType::NewLine)
            || matches!(self.prog_type, ProgrammingLineType::Unknown)
        {
            return;
        }

        self.original_line.hash(hasher);
        self.hash = hasher.finish();
    }

    pub fn get_comment(&self) -> &str {
        return match self.commented_line {
            Some(cmt) => unsafe { &*cmt },
            None => "",
        };
    }

    pub fn get_code(&self) -> &str {
        return match self.code_line {
            Some(code_ln) => unsafe { &*code_ln },
            None => "",
        };
    }

    //TODO: Find better method name
    pub fn debug_ptrs(&self) -> (Option<&str>, Option<&str>) {
        let code_line = match self.code_line {
            Some(code_ln) => Some(unsafe { &*code_ln }),
            None => None,
        };

        let commented_line = match self.commented_line {
            Some(cmt_ln) => Some(unsafe { &*cmt_ln }),
            None => None,
        };

        return (code_line, commented_line);
    }
}
