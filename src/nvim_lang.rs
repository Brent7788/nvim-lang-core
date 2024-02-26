use log::{debug, warn};

use crate::{
    common::{LOWER_CASE_ALPHABET, UPPER_CASE_ALPHABET},
    lang_tool::{
        LangTooContextTrait, LangToolTrait, LanguageToolFile, LanguageToolLines,
        LanguageToolLinesType,
    },
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

    // TODO: Find better name
    pub fn create(lang_tool_file: &LanguageToolFile) -> NvimLanguageFile {
        let mut nvim_core = NvimLanguageFile {
            file_path: lang_tool_file.prog_file.file_path.to_owned(),
            nvim_lang_lines: Vec::new(),
        };

        for language_tool_lines in &lang_tool_file.lines {
            let matches = match language_tool_lines.lang_tool.get_matches() {
                Some(matches) => matches,
                None => continue,
            };

            for lang_match in matches {
                nvim_core.push_if_comments(lang_match, language_tool_lines);
                nvim_core.push_if_code(lang_match, language_tool_lines);
                nvim_core.push_if_strings(lang_match, language_tool_lines);
            }
        }

        return nvim_core;
    }

    pub fn is_empty(&self) -> bool {
        return self.file_path.is_empty() || self.nvim_lang_lines.is_empty();
    }

    fn push_if_comments(&mut self, lang_match: &Matche, lang_tool_lines: &LanguageToolLines) {
        if !matches!(lang_tool_lines.tp, LanguageToolLinesType::Comment) {
            return;
        }

        let context = &lang_match.context;
        let chunk = context.get_incorrect_chunk();

        // debug!("CHUNk === *{}*{}", chunk, lang_match.sentence);

        if chunk.is_empty() {
            // TODO: Find better warning message
            warn!("One of the matches is empty");
            return;
        }

        for (index, line) in lang_tool_lines.prog_lines.iter().enumerate() {
            // TODO: Need to test this with long indenting
            if !(lang_match.offset <= lang_tool_lines.line_end_offset[index]) {
                continue;
            }

            let start_columns = get_target_offsets(&line.original_line, chunk);

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

            break;
        }
    }

    fn push_if_code(&mut self, lang_match: &Matche, lang_tool_lines: &LanguageToolLines) {
        if !matches!(lang_tool_lines.tp, LanguageToolLinesType::Code) {
            return;
        }

        if !matches!(
            NvimLangLineType::get_type(&lang_match.rule.category),
            NvimLangLineType::Typos
        ) {
            return;
        }

        let context = &lang_match.context;
        let chunk = context.get_incorrect_chunk();

        for line in &lang_tool_lines.prog_lines {
            let start_columns = get_target_offsets(&line.original_line, chunk);

            if start_columns.is_empty() {
                continue;
            }

            for start_column in start_columns {
                if self.line_exit(
                    line.line_number,
                    start_column,
                    start_column + context.length,
                ) {
                    continue;
                }

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

    fn push_if_strings(&mut self, lang_match: &Matche, lang_tool_lines: &LanguageToolLines) {
        if !matches!(lang_tool_lines.tp, LanguageToolLinesType::String) {
            return;
        }

        let context = &lang_match.context;
        let chunk = context.get_incorrect_chunk();

        for line in &lang_tool_lines.prog_lines {
            let start_columns = get_target_offsets(&line.original_line, chunk);

            if start_columns.is_empty() {
                continue;
            }

            for start_column in start_columns {
                if self.line_exit(
                    line.line_number,
                    start_column,
                    start_column + context.length,
                ) {
                    continue;
                }

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

    fn line_exit(&self, line_number: usize, start_column: usize, end_column: usize) -> bool {
        for nvim_lang_line in &self.nvim_lang_lines {
            if nvim_lang_line.line_number == line_number
                && nvim_lang_line.start_column == start_column
                && nvim_lang_line.end_column == end_column
            {
                return true;
            }
        }

        return false;
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
    Typograph,
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
            "TYPOGRAPHY" => NvimLangLineType::Typograph,
            _ => NvimLangLineType::Other,
        };
    }
}

fn is_first_char_part_of_alpb(sen: &str) -> bool {
    if sen.is_empty() {
        return false;
    }

    let first_char = &(sen.as_bytes()[0] as char);

    for alpb in LOWER_CASE_ALPHABET {
        if alpb == first_char {
            return true;
        }
    }

    for alpb in UPPER_CASE_ALPHABET {
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
