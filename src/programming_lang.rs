use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use log::{error, info, warn};

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

// TODO: Find better name
#[derive(Debug)]
pub struct ProgrammingLanguage<'lang> {
    pub extension: &'lang str,
    comment_delimiter: &'lang str,
    block_comment_delimiter_start: &'lang str,
    block_comment_delimiter_end: &'lang str,
    lang_type: ProgrammingLanguageType,
}

impl<'lang> ProgrammingLanguage<'lang> {
    pub fn init() -> Vec<ProgrammingLanguage<'lang>> {
        let mut programming_languages: Vec<ProgrammingLanguage> = Vec::with_capacity(2);

        programming_languages.push(ProgrammingLanguage {
            extension: ".lua",
            comment_delimiter: "--",
            block_comment_delimiter_start: "",
            block_comment_delimiter_end: "",
            lang_type: ProgrammingLanguageType::Lua,
        });

        programming_languages.push(ProgrammingLanguage {
            extension: ".rs",
            comment_delimiter: "//",
            block_comment_delimiter_start: "/*",
            block_comment_delimiter_end: "*/",
            lang_type: ProgrammingLanguageType::Rust,
        });

        return programming_languages;
    }
}

// TODO: Find better name
#[derive(Debug)]
pub struct ProgrammingFile<'pf> {
    file_path: &'pf str,
    lang: &'pf ProgrammingLanguage<'pf>,
    pub lines: Vec<ProgrammingLine>,
}

impl<'pf> ProgrammingFile<'pf> {
    pub fn new(file_path: &'pf str, lang: &'pf ProgrammingLanguage) -> Self {
        return ProgrammingFile {
            file_path,
            lines: Vec::new(),
            lang,
        };
    }

    pub fn process_lines(&mut self) {
        let file_result = File::open(self.file_path);

        let file = match file_result {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open file: {}, error: {}", self.file_path, e);
                return;
            }
        };

        let file_buf_reader = BufReader::new(file);
        let mut is_block_comment = false;

        for (index, line_res) in file_buf_reader.lines().enumerate() {
            let line = match line_res {
                Ok(line) => line,
                Err(e) => {
                    warn!("Unable to read file line. {}", e);
                    continue;
                }
            };

            let mut programming_line = ProgrammingLine::new(index + 1, line, self.lang);
            programming_line.set_type(self.lang, self.lines.last());
            /*             let line = &programming_line.original_line;

            if !is_block_comment {
                is_block_comment = line.contains(self.lang.block_comment_delimiter_start);
            }

            if is_block_comment {
                is_block_comment = !line.contains(self.lang.block_comment_delimiter_end);
                programming_line.set_commented();
                programming_line.prog_type = ProgrammingLineType::BlockComment;
                self.lines.push(programming_line);
                continue;
            } */

            // programming_line.set_commented_and_code_line(self.lang);

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
    pub line_number: usize,
    pub original_line: String,
    pub commented_line: Option<*const str>,
    pub code_line: Option<*const str>,
    pub prog_type: ProgrammingLineType,
}

impl ProgrammingLine {
    pub fn new(line_number: usize, original_line: String, lang: &ProgrammingLanguage) -> Self {
        let mut prog_type = ProgrammingLineType::Unknown;
        /*
        if original_line.is_empty() {
            prog_type = ProgrammingLineType::NewLine;
        } else if original_line.contains(lang.block_comment_delimiter_start) {
            prog_type = ProgrammingLineType::BlockCommentStart
        } */

        return ProgrammingLine {
            line_number,
            original_line,
            commented_line: None,
            code_line: None,
            prog_type,
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

    pub fn set_commented(&mut self) {
        if self.original_line.is_empty() {
            return;
        }

        self.commented_line = Some(self.original_line.as_str());
    }

    pub fn set_commented_and_code_line(&mut self, lang: &ProgrammingLanguage) {
        if self.original_line.is_empty() {
            return;
        }

        let line_comment_option = self.original_line.split_once(lang.comment_delimiter);

        match line_comment_option {
            Some((left_of_line, right_of_line)) => {
                self.commented_line = Some(right_of_line);

                if left_of_line.trim().is_empty() {
                    self.prog_type = ProgrammingLineType::Comment;
                    return;
                }
                // TODO: The code line need to be transformed
                self.code_line = Some(left_of_line);
                self.prog_type = ProgrammingLineType::CodeWithComment;
            }
            None => {
                self.code_line = Some(self.original_line.as_str());
                self.prog_type = ProgrammingLineType::Code;
            }
        }
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
