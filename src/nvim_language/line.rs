use std::{
    hash::{DefaultHasher, Hash},
    sync::Arc,
};

use languagetool_rust::check::{Category, Match};
use serde::{Deserialize, Serialize};

use crate::{
    code::code_file::CodeLine,
    common::{LOWER_CASE_ALPHABET, UPPER_CASE_ALPHABET},
    language_tool::{language_tool_file::LanguageToolLineType, LanguageToolContextTrait},
    nvim_lang_dictionary::NvimLanguageReadonlyDictionary,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NvimLanguageLine {
    pub line_number: usize,
    pub start_column: usize,
    pub end_column: usize,
    pub options: NvimOptions,
    pub data_type: NvimLangLineType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NvimOptions {
    pub original: String,
    pub options: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NvimLangLineType {
    Typos,
    Punctuation,
    ConfusedWords,
    Redundancy,
    Casing,
    Grammar,
    Misc,
    Semantics,
    Typography,
    Other,
}

impl PartialEq for NvimLanguageLine {
    fn eq(&self, other: &Self) -> bool {
        return self.line_number == other.line_number
            && self.start_column == other.start_column
            && self.end_column == other.end_column;
    }
}

impl Eq for NvimLanguageLine {}

impl Hash for NvimLanguageLine {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.line_number.hash(state);
        self.start_column.hash(state);
        self.end_column.hash(state);
    }
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
            "TYPOGRAPHY" => NvimLangLineType::Typography,
            _ => NvimLangLineType::Other,
        };
    }
}

impl NvimLanguageLine {
    pub async fn new_lines(
        lang_match: Match,
        language_tool_line_type: Arc<LanguageToolLineType>,
        language_dictionary: Arc<NvimLanguageReadonlyDictionary>,
    ) -> Option<Vec<NvimLanguageLine>> {
        let context = &lang_match.context;
        let chunk = context.get_incorrect_chunk();

        if chunk.is_empty() {
            return None;
        }

        if language_dictionary.exit_in_dictionary(chunk) {
            return None;
        }

        let mut nvim_language_lines = Vec::new();

        match *language_tool_line_type {
            LanguageToolLineType::Block(ref code_block) => {
                for code_line in &code_block.lines {
                    match NvimLanguageLine::code_line_to_nvim_lang_lines(
                        code_line,
                        chunk,
                        context.length,
                        &lang_match,
                    ) {
                        Some(nvim_lang_lines) => nvim_language_lines.extend(nvim_lang_lines),
                        None => {}
                    };
                }
            }
            LanguageToolLineType::Code(ref codes) => {
                for code in codes {
                    match NvimLanguageLine::code_line_to_nvim_lang_lines(
                        &code.line,
                        chunk,
                        context.length,
                        &lang_match,
                    ) {
                        Some(nvim_lang_lines) => nvim_language_lines.extend(nvim_lang_lines),
                        None => {}
                    };
                }
            }
        }

        if nvim_language_lines.is_empty() {
            return None;
        }

        return Some(nvim_language_lines);
    }

    fn code_line_to_nvim_lang_lines(
        code_line: &CodeLine,
        chunk: &str,
        context_len: usize,
        lang_match: &Match,
    ) -> Option<Vec<NvimLanguageLine>> {
        let mut nvim_lang_lines = Vec::new();
        let start_columns = get_target_offsets(&code_line.original_line, chunk);

        if start_columns.is_empty() {
            return None;
        }

        for start_column in start_columns {
            nvim_lang_lines.push(NvimLanguageLine {
                line_number: code_line.line_number,
                start_column,
                end_column: start_column + context_len,
                options: NvimOptions {
                    original: chunk.to_owned(),
                    options: lang_match
                        .replacements
                        .iter()
                        .map(|r| r.value.clone())
                        .take(20) // TODO: Set 20 as const
                        .collect(),
                },
                data_type: NvimLangLineType::get_type(&lang_match.rule.category),
            });
        }

        return Some(nvim_lang_lines);
    }
}

// BUG: Does not work on this:
//      `local TEXT = "a shop with brances in many places, especialy one selling a specific type of prduct.";`
//      The `a` in local is detected and not in the string.
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

fn is_not_valid_offset(input_index: usize, input: &Vec<&str>) -> bool {
    //INFO: The last input offset is always invalid
    if input_index == input.len() - 1 {
        return true;
    }

    let before_index = input_index as isize - 1;
    let after_index = input_index + 1;

    if before_index == -1 {
        return is_char_not_valid(input[after_index]);
    }

    if after_index == input.len() {
        return is_char_not_valid(input[before_index as usize]);
    }

    return is_char_not_valid(input[before_index as usize])
        || is_char_not_valid(input[after_index]);
}

fn is_char_not_valid(sen: &str) -> bool {
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

    // HACK:
    // if first_char == &'\'' {
    //     return true;
    // }

    return false;
}
