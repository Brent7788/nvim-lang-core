use log::{debug, warn};

use crate::{
    lang_tool::LanguageToolFile,
    modules::{Category, Matche},
};

#[derive(Debug)]
pub struct NvimLanguageFile {
    pub file_path: String,
    pub nvim_lang_lines: Vec<NvimLanguageLine>,
}

impl NvimLanguageFile {
    pub fn new() -> Self {
        return NvimLanguageFile {
            file_path: String::new(),
            nvim_lang_lines: Vec::new(),
        };
    }

    pub fn create(lang_tool_file: &LanguageToolFile) -> NvimLanguageFile {
        let mut nvim_core = NvimLanguageFile {
            file_path: lang_tool_file.prog_file.file_path.to_owned(),
            nvim_lang_lines: Vec::new(),
        };

        for comment in &lang_tool_file.comments {
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

                    let start_columns = get_target_offsets(&line.original_line, chunk);

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
                        nvim_core.nvim_lang_lines.push(NvimLanguageLine {
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

        nvim_core.process_code(lang_tool_file);

        return nvim_core;
    }

    pub fn is_empty(&self) -> bool {
        return self.file_path.is_empty() || self.nvim_lang_lines.is_empty();
    }

    // TODO: Find better name
    fn process_code(&mut self, lang_tool_file: &LanguageToolFile) {
        for code_line in &lang_tool_file.code {
            let matches: &Vec<Matche> = match code_line.lang_tool {
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

                let line = code_line.prog_line;
                let start_columns = get_target_offsets(&line.original_line, chunk);

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
                    self.nvim_lang_lines.push(NvimLanguageLine {
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
            }
        }
    }
}

#[derive(Debug)]
pub struct NvimLanguageLine {
    pub line_number: usize,
    pub start_column: usize,
    pub end_column: usize,
    pub options: NvimOptions,
    pub data_type: NvimLangLineType,
}

#[derive(Debug)]
pub struct NvimOptions {
    pub original: String,
    pub options: Vec<String>,
}

#[derive(Debug)]
pub enum NvimLangLineType {
    Typos,
    Punctuation,
    ConfusedWords,
    Redundancy,
    Casing,
    Grammar,
    Misc,
    Semantics,
    Other,
}

impl NvimLangLineType {
    pub fn get_type(cat: &Category) -> NvimLangLineType {
        return match cat.id.as_str() {
            "TYPOS" => NvimLangLineType::Typos,
            "PUNCTUATION" => NvimLangLineType::Punctuation,
            "CONFUSED_WORDS" => NvimLangLineType::ConfusedWords,
            "REDUNDANCY" => NvimLangLineType::Redundancy,
            "CASING" => NvimLangLineType::Casing,
            "GRAMMAR" => NvimLangLineType::Grammar,
            "MISC" => NvimLangLineType::Misc,
            "SEMANTICS" => NvimLangLineType::Semantics,
            _ => NvimLangLineType::Other,
        };
    }
}

const ALPHABET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn is_first_char_part_of_alpb(sen: &str) -> bool {
    if sen.is_empty() {
        return false;
    }

    let first_char = &(sen.as_bytes()[0] as char);

    for alpb in ALPHABET {
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
        return is_first_char_part_of_alpb(input[after_index]);
    }

    if after_index == input.len() {
        return is_first_char_part_of_alpb(input[before_index as usize]);
    }

    return is_first_char_part_of_alpb(input[before_index as usize])
        || is_first_char_part_of_alpb(input[after_index]);
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

        if is_not_valid_offset(index, &input_collection) {
            continue;
        }

        offsets.push(current_offset);
    }

    return offsets;
}
