use std::sync::{Arc, Mutex, MutexGuard};

use log::warn;

use crate::lang_tool::LanguageToolFile;
use crate::lang_tool_client::{LangToolClient, LanguageToolClientState};
use crate::nvim_lang::NvimLanguageFile;
use crate::nvim_lang_dictionary::NvimLanguageDictionary;
use crate::programming_lang::{ProgrammingFile, ProgrammingLanguage};

#[derive(Debug)]
pub struct NvimLangCore<'lang> {
    lang_tool_client: Arc<Mutex<LangToolClient>>,
    programming_languages: [ProgrammingLanguage<'lang>; 2],
}

impl<'lang> NvimLangCore<'lang> {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> NvimLangCore<'lang> {
        return NvimLangCore {
            lang_tool_client: Arc::new(Mutex::new(LangToolClient::new(lang_tool_url, lang))),
            programming_languages: ProgrammingLanguage::init(),
        };
    }

    pub fn get_language_tool_client(&self) -> LanguageToolClientState {
        let language_tool_client = self.lang_tool_client.try_lock();

        return match language_tool_client {
            Ok(client) => LanguageToolClientState::MainGuard(client),
            Err(e) => {
                warn!("Unable to get LanguageTool client: Error: {:#?}", e);
                // TODO: Remember to handle URL and port here.
                return LanguageToolClientState::Default(LangToolClient::new(None, None));
            }
        };
    }

    pub fn spawn_blocking<F, R>(&self, future: F)
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        match self.get_language_tool_client() {
            LanguageToolClientState::MainGuard(g) => {
                match g.tokio_runtime.as_ref() {
                    Some(tokio_runtime) => {
                        tokio_runtime.spawn_blocking(future);
                    }
                    None => warn!("Unable to run function because there is no Tokio Runtime in the language tool client."),
                };
            }
            LanguageToolClientState::Default(d) => {
                match d.tokio_runtime.as_ref() {
                    Some(tokio_runtime) => {
                        tokio_runtime.spawn_blocking(future);
                    }
                    None => warn!("Unable to run function because there is no Tokio Runtime in the default language tool client."),
                };
            }
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

        let lang_tool_client = self.get_language_tool_client();

        let lang_tool_file = match lang_tool_client {
            LanguageToolClientState::MainGuard(languagetool_client) => {
                if let None = languagetool_client.tokio_runtime {
                    return NvimLanguageFile::new();
                }

                LanguageToolFile::new(&prog_file, &nvim_language_dictionary, &languagetool_client)
            }
            LanguageToolClientState::Default(languagetool_client) => {
                if let None = languagetool_client.tokio_runtime {
                    return NvimLanguageFile::new();
                }

                LanguageToolFile::new(&prog_file, &nvim_language_dictionary, &languagetool_client)
            }
        };

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
