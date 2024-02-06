use log::{debug, warn};

use crate::nvim_lang::NvimLangLineType;
use crate::{
    lang_tool_client::LangToolClient,
    modules::{LangTool, Matche},
    nvim_lang::{NvimLanguageFile, NvimLanguageLine, NvimOptions},
    programming_lang::{ProgrammingFile, ProgrammingLine},
};

#[derive(Debug)]
pub struct LanguageToolFile<'ltc> {
    prog_file: &'ltc ProgrammingFile<'ltc>,
    comments: Vec<Comment<'ltc>>,
}

impl<'ltc> LanguageToolFile<'ltc> {
    pub async fn new(
        prog_file: &'ltc ProgrammingFile<'ltc>,
        client: &LangToolClient,
    ) -> LanguageToolFile<'ltc> {
        return LanguageToolFile {
            prog_file,
            comments: Comment::generate(prog_file, client).await,
        };
    }

    pub fn generate_nvim_language_file(&self) -> NvimLanguageFile {
        let mut nvim_core = NvimLanguageFile {
            file_path: self.prog_file.file_path.to_owned(),
            data: Vec::new(),
        };

        // info!("{:#?}", self);

        for comment in &self.comments {
            // debug!("COMMENT = {:#?}", comment);

            let matches: &Vec<Matche> = match comment.lang_tool {
                Some(ref lang_tool) => {
                    if lang_tool.matches.is_empty() {
                        continue;
                    }

                    &lang_tool.matches
                }
                None => continue,
            };

            debug!("MATCH COUNT: {}", matches.len());

            for lang_match in matches {
                let context = &lang_match.context;
                let offset = context.offset;
                let lenth = context.offset + context.length;
                let chunk: &str = &context.text[offset..lenth];

                // debug!("CHUNk === *{}*{}", chunk, lang_match.sentence);

                if chunk.is_empty() {
                    // TODO: Find better warning message
                    warn!("One of the matches is empty");
                    continue;
                }

                for (index, line) in comment.prog_lines.iter().enumerate() {
                    /*                     debug!(
                                           "OR: {} -{}- {}",
                                           line.line_number, chunk, line.original_line,
                                       );
                    */
                    // TODO: Need to test this with long indenting
                    if !(lang_match.offset <= comment.line_end_offset[index]) {
                        continue;
                    }

                    let start_columns =
                        LanguageToolFile::get_target_offsets(&line.original_line, chunk);

                    // debug!("Columns {:?}", start_columns);

                    if start_columns.is_empty() {
                        warn!(
                            "Was unable to get offset off word {} in line {}",
                            chunk, line.line_number
                        );
                        continue;
                    }

                    // TODO: Need to check for duplicates
                    for start_column in start_columns {
                        nvim_core.data.push(NvimLanguageLine {
                            line_number: line.line_number,
                            start_column,
                            end_column: start_column + context.length,
                            options: NvimOptions {
                                original: chunk.to_owned(),
                                options: lang_match
                                    .replacements
                                    .iter()
                                    .map(|r| r.value.clone())
                                    .collect(), // TODO: There should be max limit on the options!
                            },
                            data_type: NvimLangLineType::get_type(&lang_match.rule.category),
                        });
                    }

                    break;
                }
            }
        }

        return nvim_core;
    }

    const ALPHABET: &[char] = &[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    fn is_first_char_part_of_alpb(sen: &str) -> bool {
        if sen.is_empty() {
            return false;
        }

        let first_char = &(sen.as_bytes()[0] as char);

        for alpb in LanguageToolFile::ALPHABET {
            if alpb == first_char {
                return true;
            }
        }

        return false;
    }

    fn is_not_valid_offset(input_indext: usize, input: &Vec<&str>) -> bool {
        //INFO: The last input offset is always invalid
        if input_indext == input.len() - 1 {
            return true;
        }

        let before_index = input_indext as isize - 1;
        let after_index = input_indext + 1;

        if before_index == -1 {
            return LanguageToolFile::is_first_char_part_of_alpb(input[after_index]);
        }

        if after_index == input.len() {
            return LanguageToolFile::is_first_char_part_of_alpb(input[before_index as usize]);
        }

        return LanguageToolFile::is_first_char_part_of_alpb(input[before_index as usize])
            || LanguageToolFile::is_first_char_part_of_alpb(input[after_index]);
    }

    fn get_target_offsets(input_string: &str, target: &str) -> Vec<usize> {
        let input_collection: Vec<&str> = input_string.split(target).collect();
        let mut offsets: Vec<usize> = Vec::new();

        if input_collection.is_empty() || input_collection.len() == 1 {
            return offsets;
        }

        let mut current_offset: usize = 0;

        for (index, input) in input_collection.iter().enumerate() {
            current_offset += (*input).len();

            if index > 0 {
                current_offset += target.len();
            }

            if LanguageToolFile::is_not_valid_offset(index, &input_collection) {
                continue;
            }

            offsets.push(current_offset);
        }

        return offsets;
    }
}

#[derive(Debug)]
struct Comment<'c> {
    prog_lines: Vec<&'c ProgrammingLine>,
    line_end_offset: Vec<usize>,
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

            comment.comment = format!("{} {}", comment.comment.as_str(), prog_line.get_comment());

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

        let offset = prog_line.get_comment().len() + last_line_end_offset;

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
