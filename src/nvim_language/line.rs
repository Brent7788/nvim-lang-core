use std::{hash::Hash, sync::Arc};

use languagetool_rust::check::{Category, Match};
use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{
    code::code_file::{CodeBlock, CodeLine, CodeType},
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
    pub async fn new(
        lang_match: Match,
        language_tool_line_type: Arc<LanguageToolLineType>,
        language_dictionary: Arc<NvimLanguageReadonlyDictionary>,
    ) -> Option<NvimLanguageLine> {
        let context = &lang_match.context;
        let chunk = context.get_incorrect_chunk();

        if chunk.is_empty() {
            return None;
        }

        if language_dictionary.exit_in_dictionary(chunk) {
            return None;
        }

        match *language_tool_line_type {
            LanguageToolLineType::Block(ref code_block) => {
                return NvimLanguageLine::code_block_to_nvim_lang_line(
                    code_block,
                    chunk,
                    &lang_match,
                );
            }
            LanguageToolLineType::Code(ref code) => {
                // INFO: For code type values ignore grammer and accept only spelling mustakes.
                if let CodeType::Code = code.tp {
                    let nvim_lang_line_type = NvimLangLineType::get_type(&lang_match.rule.category);

                    if !matches!(nvim_lang_line_type, NvimLangLineType::Typos) {
                        return None;
                    }
                }

                return NvimLanguageLine::code_line_to_nvim_lang_line(
                    &code.line,
                    chunk,
                    &lang_match,
                    lang_match.offset,
                );
            }
        }

        return None;
    }

    fn code_block_to_nvim_lang_line(
        code_block: &CodeBlock,
        chunk: &str,
        lang_match: &Match,
    ) -> Option<NvimLanguageLine> {
        let nvim_lang_line_type = NvimLangLineType::get_type(&lang_match.rule.category);

        let mut absolute_len = 0;
        let mut flage = false;

        for code_line in &code_block.lines {
            if absolute_len > lang_match.offset {
                break;
            }

            let start_column = lang_match.offset - absolute_len;

            // INFO: Increste the absolute length and add one that is the line breack
            if flage && absolute_len == 0 {
                absolute_len = code_line.original_line.trim_start().len() + 1;
            } else if flage {
                absolute_len = absolute_len + code_line.original_line.len() + 1;
            }

            // INFO: Subtract starting delimiter from the absolute length.
            if !flage && absolute_len == 0 {
                absolute_len = code_line.original_line.trim().len()
                    - code_block
                        .code_block_current_line_syntax
                        .start_delimiter
                        .len();
                flage = true;
            }

            // INFO: Ignore all zero start column typography on code blocks
            if start_column == 0 && matches!(nvim_lang_line_type, NvimLangLineType::Typography) {
                return None;
            }

            match NvimLanguageLine::code_line_to_nvim_lang_line(
                code_line,
                chunk,
                lang_match,
                start_column,
            ) {
                Some(l) => return Some(l),
                None => continue,
            }
        }

        return None;
    }

    fn code_line_to_nvim_lang_line(
        code_line: &CodeLine,
        chunk: &str,
        lang_match: &Match,
        start_column: usize,
    ) -> Option<NvimLanguageLine> {
        if start_column > code_line.original_line.len() {
            return None;
        }

        let end_chunk_index = start_column + lang_match.length;

        if end_chunk_index > code_line.original_line.len() {
            return None;
        }

        for start_column in start_column..code_line.original_line.len() {
            let chunk_check =
                &code_line.original_line[start_column..(start_column + lang_match.length)];

            if chunk_check != chunk {
                continue;
            }

            return Some(NvimLanguageLine {
                line_number: code_line.line_number,
                start_column,
                end_column: start_column + lang_match.length,
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

        return None;
    }
}

pub fn get_target_offsets2(input_string: &str, target: &str) -> Vec<usize> {
    let input_split = input_string.split_whitespace();

    let mut offsets: Vec<usize> = Vec::new();
    let mut current_offset: usize = 0;

    for input in input_split {
        if input.is_empty() || input.len() < target.len() {
            current_offset += input.len() + 1;
            continue;
        }

        if input == target {
            offsets.push(current_offset);
        }

        current_offset += input.len() + 1;
    }

    return offsets;
}
