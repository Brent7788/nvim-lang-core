use std::{
    fs::File,
    io::{BufRead, BufReader},
    marker::PhantomData,
};

use log::{error, info, warn};

#[derive(Debug)]
pub enum ProgrammingLanguageType {
    Lua,
    Rust,
}

// TODO: Find better name
#[derive(Debug)]
pub struct ProgrammingLanguage<'ext, 'cmt, 'cmt_st, 'cmt_end> {
    extension: &'ext str,
    comment_delimiter: &'cmt str,
    block_comment_delimiter_start: &'cmt_st str,
    block_comment_delimiter_end: &'cmt_end str,
    lang_type: ProgrammingLanguageType,
}

impl<'ext, 'cmt, 'cmt_st, 'cmt_end> ProgrammingLanguage<'ext, 'cmt, 'cmt_st, 'cmt_end> {
    pub fn init() -> Vec<ProgrammingLanguage<'ext, 'cmt, 'cmt_st, 'cmt_end>> {
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
pub struct ProgrammingLine {
    original_line: String,
    commented_line: Option<String>,
    code_line: Option<String>,
}

impl ProgrammingLine {
    pub fn new(original_line: String) -> Self {
        return ProgrammingLine {
            original_line,
            commented_line: None,
            code_line: None,
        };
    }
}

// TODO: Find better name
#[derive(Debug)]
pub struct ProgrammingFile<'fp> {
    file_path: &'fp str,
    line: Vec<ProgrammingLine>,
}

impl<'fp> ProgrammingFile<'fp> {
    pub fn new(file_path: &'fp str) -> Self {
        return ProgrammingFile {
            file_path,
            line: Vec::new(),
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

            let mut programming_line = ProgrammingLine::new(line);
            let line = &programming_line.original_line;

            if !is_block_comment {
                is_block_comment = line.contains(lang.block_comment_delimiter_start);
            } else {
                is_block_comment = !line.contains(lang.block_comment_delimiter_end);
                info!("Need to handler bloc comments");
                continue;
            }

            let line_comment_option = line.split_once(lang.comment_delimiter);

            if let Some((left_of_line, right_of_line)) = line_comment_option {
                // TODO: The code line need to be transformed
                programming_line.code_line = Some(left_of_line.to_owned());
                programming_line.commented_line = Some(right_of_line.to_owned());
            }

            self.line.push(programming_line);
        }
    }
}
