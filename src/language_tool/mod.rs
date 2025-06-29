pub mod language_tool_file;

use languagetool_rust::{
    check::{Context, Match},
    CheckResponse,
};
use log::error;

pub trait LanguageToolTrait {
    fn get_matches(&self) -> Option<&Vec<Match>>;
}

impl LanguageToolTrait for Option<CheckResponse> {
    fn get_matches(&self) -> Option<&Vec<Match>> {
        return match self {
            Some(ref lang_tool) => {
                if lang_tool.matches.is_empty() {
                    return None;
                }

                return Some(&lang_tool.matches);
            }
            None => None,
        };
    }
}

pub trait LanguageToolContextTrait {
    fn get_incorrect_chunk(&self) -> &str;
}

impl LanguageToolContextTrait for Context {
    fn get_incorrect_chunk(&self) -> &str {
        let mut offset = self.offset;
        let mut length = self.offset + self.length;

        while !self.text.is_char_boundary(offset) {
            offset += 1;
            length += 1;
        }

        if self.text.len() < length {
            error!("Char boundary incroment error in text: `{}`", self.text);
            return "";
        }

        return &self.text[offset..length];
    }
}
