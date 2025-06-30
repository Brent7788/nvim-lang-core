pub mod language_tool_file;

use languagetool_rust::{
    check::{Context, Match},
    CheckResponse,
};
use log::error;

pub trait LanguageToolTrait {
    fn get_matches(&self) -> Option<&Vec<Match>>;
}

impl LanguageToolTrait for CheckResponse {
    fn get_matches(&self) -> Option<&Vec<Match>> {
        if self.matches.is_empty() {
            return None;
        }

        return Some(&self.matches);
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
