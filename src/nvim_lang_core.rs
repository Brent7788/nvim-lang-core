use log::warn;
use reqwest::Client;

use crate::programming_lang::ProgrammingLanguage;

#[derive(Debug)]
pub struct NvimLangCore<'lang> {
    languagetool_url: String,
    language: String,
    client: Client,
    programming_languages: Vec<ProgrammingLanguage<'lang>>,
}

impl<'lang> NvimLangCore<'lang> {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> NvimLangCore<'lang> {
        let mut languagetool_url: String = "http://localhost:8081".to_owned();
        let mut language: String = "en-US".to_owned();
        let client = Client::new();

        if let Some(url) = lang_tool_url {
            languagetool_url = url;
        }

        if let Some(lang) = lang {
            language = lang;
        }

        return NvimLangCore {
            languagetool_url,
            language,
            client,
            programming_languages: ProgrammingLanguage::init(),
        };
    }

    pub fn process_file(&self, file_path: String) {
        if file_path.is_empty() {
            warn!("No file path was provided");
        }
    }

    fn get_file_type(&self, file_path: &String) -> Option<&ProgrammingLanguage> {
        for lang in &self.programming_languages {
            if file_path.ends_with(lang.extension) {
                return Some(lang);
            }
        }

        return None;
    }
}
