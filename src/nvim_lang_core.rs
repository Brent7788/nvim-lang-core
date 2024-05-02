use std::sync::MutexGuard;

use log::warn;

use crate::lang_tool::LanguageToolFile;
use crate::lang_tool_client::LangToolClient;
use crate::nvim_lang::NvimLanguageFile;
use crate::nvim_lang_dictionary::NvimLanguageDictionary;
use crate::programming_lang::{ProgrammingFile, ProgrammingLanguage};

#[derive(Debug)]
pub struct NvimLangCore<'lang> {
    lang_tool_client: LangToolClient,
    pub programming_languages: [ProgrammingLanguage<'lang>; 2],
}

impl<'lang> NvimLangCore<'lang> {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> NvimLangCore<'lang> {
        return NvimLangCore {
            lang_tool_client: LangToolClient::new(lang_tool_url, lang),
            programming_languages: ProgrammingLanguage::init(),
        };
    }

    pub fn get_language_tool_client(&self) -> &LangToolClient {
        return &self.lang_tool_client;
    }

    pub fn spawn_blocking<F, R>(&self, future: F)
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        return match self.lang_tool_client.tokio_runtime.as_ref() {
                Some(tokio_runtime) => {
                    tokio_runtime.spawn_blocking(future);
                }
                None => warn!("Unable to run function because there is no Tokio Runtime in the language tool client."),
            };
    }

    // TODO: Find better method name.
    pub fn process_file(
        &self,
        file_path: String,
        nvim_language_dictionary: Option<MutexGuard<NvimLanguageDictionary>>,
    ) -> NvimLanguageFile {
        if file_path.is_empty() {
            warn!("No file path was provided");
            return NvimLanguageFile::new();
        }

        let lang = match self.get_file_type(&file_path) {
            Some(lang) => lang,
            None => {
                warn!(
                    "nvim-lang-core does not support this file type: {}",
                    file_path
                );
                return NvimLanguageFile::new();
            }
        };

        let prog_file = ProgrammingFile::create(&file_path, &lang);

        let languagetool_client = self.get_language_tool_client();
        let lang_tool_file =
            LanguageToolFile::new(&prog_file, &nvim_language_dictionary, languagetool_client);

        return NvimLanguageFile::create(&lang_tool_file, &nvim_language_dictionary);
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
