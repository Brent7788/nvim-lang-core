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
pub struct ProgrammingFile<'fp> {
    file_path: &'fp str,
    pub lines: Vec<ProgrammingLine>,
}

impl<'fp> ProgrammingFile<'fp> {
    pub fn new(file_path: &'fp str) -> Self {
        return ProgrammingFile {
            file_path,
            lines: Vec::new(),
        };
    }

    pub fn pros(&mut self, lang: &ProgrammingLanguage) {
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

            let mut programming_line = ProgrammingLine::new(index + 1, line);
            let line = &programming_line.original_line;

            if !is_block_comment {
                is_block_comment = line.contains(lang.block_comment_delimiter_start);
            }

            if is_block_comment {
                is_block_comment = !line.contains(lang.block_comment_delimiter_end);
                programming_line.set_commented();
                self.lines.push(programming_line);
                continue;
            }

            programming_line.set_commented_and_code_line(&lang);

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
}

impl ProgrammingLine {
    pub fn new(line_number: usize, original_line: String) -> Self {
        return ProgrammingLine {
            line_number,
            original_line,
            commented_line: None,
            code_line: None,
        };
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
                    return;
                }
                // TODO: The code line need to be transformed
                self.code_line = Some(left_of_line);
            }
            None => {
                self.code_line = Some(self.original_line.as_str());
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
