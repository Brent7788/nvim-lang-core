use log::{debug, info, warn};

use crate::{
    lang_tool_client::LangToolClient,
    modules::{LangTool, Matche},
    programming_lang::{ProgrammingFile, ProgrammingLine},
};

//TODO: Find better name
#[derive(Debug)]
pub struct NvimLangCoreData {
    pub file_path: String,
    pub data: Vec<Data>,
}

impl NvimLangCoreData {
    pub fn new() -> Self {
        return NvimLangCoreData {
            file_path: String::new(),
            data: Vec::new(),
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.file_path.is_empty() || self.data.is_empty();
    }
}

//TODO: Find better name
#[derive(Debug)]
pub struct Data {
    pub line_number: usize,
    pub start_column: usize,
    pub end_column: usize,
    pub options: Options,
    pub data_type: DataType,
}

#[derive(Debug)]
pub struct Options {
    pub original: String,
    pub options: Vec<String>,
}

//TODO: Find better name
#[derive(Debug)]
pub enum DataType {
    SpellMistake,
}

//TODO: Find better name
#[derive(Debug)]
pub struct LangToolCore<'ltc> {
    prog_file: &'ltc ProgrammingFile<'ltc>,
    comments: Vec<Comment<'ltc>>,
}

impl<'ltc> LangToolCore<'ltc> {
    pub async fn new(
        prog_file: &'ltc ProgrammingFile<'ltc>,
        client: &LangToolClient,
    ) -> LangToolCore<'ltc> {
        return LangToolCore {
            prog_file,
            comments: Comment::generate(prog_file, client).await,
        };
    }

    //TODO: Find better name
    pub fn get_data(&self) -> NvimLangCoreData {
        let mut nvim_core = NvimLangCoreData {
            file_path: self.prog_file.file_path.to_owned(),
            data: Vec::new(),
        };

        // info!("{:#?}", self);

        for comment in &self.comments {
            debug!("COMMENT = {:#?}", comment);

            let matches: &Vec<Matche> = match comment.lang_tool {
                Some(ref lang_tool) => {
                    if lang_tool.matches.is_empty() {
                        continue;
                    }

                    &lang_tool.matches
                }
                None => continue,
            };

            for lang_match in matches {
                let context = &lang_match.context;
                let offset = context.offset;
                let lenth = context.offset + context.length;
                let chunk: &str = &context.text[offset..lenth];

                if chunk.is_empty() {
                    // TODO: Find better warning message
                    warn!("One of the matches is empty");
                    continue;
                }

                for line in &comment.prog_lines {
                    if !line.original_line.contains(chunk) {
                        continue;
                    }

                    if !line.original_line.contains(&lang_match.sentence) {
                        continue;
                    }

                    // TODO: Need to check if there is multiple words wrong on the same line
                    let start_column = match self.find_target_offset(&line.original_line, chunk) {
                        Some(start_column) => start_column,
                        None => {
                            warn!(
                                "Was unable to get offset off word {} in line {}",
                                chunk, line.line_number
                            );
                            continue;
                        }
                    };

                    debug!(
                        "OR: {} -{}- {}",
                        line.line_number, chunk, line.original_line
                    );

                    nvim_core.data.push(Data {
                        line_number: line.line_number,
                        start_column,
                        end_column: start_column + context.length,
                        options: Options {
                            original: chunk.to_owned(),
                            options: lang_match
                                .replacements
                                .iter()
                                .map(|r| r.value.clone())
                                .collect(), // TODO: There should be max limit on the options!
                        },
                        data_type: DataType::SpellMistake,
                    })
                }
            }
        }

        return nvim_core;
    }

    //BUG: What if the target work is between multiple characters?
    //     What if there is multiple targets in the same input string?
    fn find_target_offset(&self, input_string: &str, target: &str) -> Option<usize> {
        let n = input_string.split_once(target);

        return match n {
            Some((left, _)) => {
                return Some(left.len());
            }
            None => None,
        };
    }
}

#[derive(Debug)]
struct Comment<'c> {
    prog_lines: Vec<&'c ProgrammingLine>,
    line_end_offset: Vec<usize>, // TODO: This is not being used.
    comment: String,
    lang_tool: Option<LangTool>,
}

impl<'c> Comment<'c> {
    fn new() -> Comment<'c> {
        return Comment {
            prog_lines: Vec::new(),
            line_end_offset: Vec::new(),
            comment: String::new(),
            lang_tool: None,
        };
    }
    async fn generate<'pl>(
        prog_file: &'pl ProgrammingFile<'pl>,
        client: &LangToolClient,
    ) -> Vec<Comment<'pl>> {
        let mut comments: Vec<Comment> = Vec::new();

        let mut comment: Comment = Comment::new();

        for prog_line in &prog_file.lines {
            if !Comment::is_line_comment(prog_line) && !comment.is_empty() {
                comment.lang_tool = client.get_lang_tool(&comment.comment).await;
                comments.push(comment);
                comment = Comment::new();
                continue;
            } else if !Comment::is_line_comment(prog_line) && comment.is_empty() {
                continue;
            }

            comment.push_line_end_offset(prog_line);

            //TODO: Need to remove the trailing line break
            comment.comment = format!("{}\n{}", comment.comment.as_str(), prog_line.get_comment());

            comment.prog_lines.push(prog_line);

            // info!("WHAT COMMENT: {:#?}", comment);
        }

        if comment.prog_lines.len() > 0 {
            comment.lang_tool = client.get_lang_tool(&comment.comment).await;
            comments.push(comment);
        }

        // info!("COMMENT: {:#?}", comments);

        return comments;
    }

    fn is_line_comment(prog_line: &ProgrammingLine) -> bool {
        return match prog_line.prog_type {
            crate::programming_lang::ProgrammingLineType::CodeWithComment => true,
            crate::programming_lang::ProgrammingLineType::Comment => true,
            crate::programming_lang::ProgrammingLineType::BlockCommentStart => true,
            crate::programming_lang::ProgrammingLineType::BlockComment => true,
            crate::programming_lang::ProgrammingLineType::BlockCommentEnd => true,
            crate::programming_lang::ProgrammingLineType::BlockCommentStartAndEnd => true,
            _ => false,
        };
    }

    fn push_line_end_offset(&mut self, prog_line: &ProgrammingLine) {
        let last_line_end_offset = match self.line_end_offset.last() {
            Some(ln_end) => ln_end,
            None => &0,
        };

        let offset = prog_line.original_line.len() - 1 + last_line_end_offset;

        self.line_end_offset.push(offset);
    }

    fn is_empty(&self) -> bool {
        if self.prog_lines.is_empty() && self.line_end_offset.is_empty() && self.comment.is_empty()
        {
            return true;
        }

        return false;
    }
}
