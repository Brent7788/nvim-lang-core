use crate::modules::Category;

#[derive(Debug)]
pub struct NvimLanguageFile {
    pub file_path: String,
    pub data: Vec<NvimLanguageLine>,
}

impl NvimLanguageFile {
    pub fn new() -> Self {
        return NvimLanguageFile {
            file_path: String::new(),
            data: Vec::new(),
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.file_path.is_empty() || self.data.is_empty();
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
