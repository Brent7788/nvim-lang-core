use log::{info, warn};
use reqwest::Client;

use crate::programming_lang::{ProgrammingFile, ProgrammingLanguage};

// TODO: Should be in its own file.
#[derive(Debug)]
pub struct LangToolClient {
    pub languagetool_url: String,
    pub language: String,
    pub client: Client,
}

impl LangToolClient {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> Self {
        let mut languagetool_url: String = "http://localhost:8081".to_owned();
        let mut language: String = "en-US".to_owned();
        let client = Client::new();

        if let Some(url) = lang_tool_url {
            languagetool_url = url;
        }

        if let Some(lang) = lang {
            language = lang;
        }

        return LangToolClient {
            languagetool_url,
            language,
            client,
        };
    }
}

#[derive(Debug)]
pub struct NvimLangCore<'lang> {
    lang_tool_client: LangToolClient,
    programming_languages: [ProgrammingLanguage<'lang>; 2],
}

impl<'lang> NvimLangCore<'lang> {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> NvimLangCore<'lang> {
        return NvimLangCore {
            lang_tool_client: LangToolClient::new(lang_tool_url, lang),
            programming_languages: ProgrammingLanguage::init(),
        };
    }

    pub fn process_file(&self, file_path: String) {
        if file_path.is_empty() {
            warn!("No file path was provided");
            return;
        }

        let lang = match self.get_file_type(&file_path) {
            Some(lang) => lang,
            None => {
                warn!(
                    "nvim-lang-core does not support this file type: {}",
                    file_path
                );
                return;
            }
        };

        let mut prog_file = ProgrammingFile::new(&file_path, &lang);

        prog_file.process_lines();

        info!("{:#?}", prog_file);
        prog_file.debug_p();
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
