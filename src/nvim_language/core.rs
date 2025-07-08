use std::{future::Future, sync::Arc};

use log::warn;

use crate::{
    code::{
        code_file::CodeFile,
        programming::{ProgrammingLanguageType, LUA, RUST},
    },
    lang_tool_client::LangToolClient,
    language_tool::language_tool_file::LanguageToolFile,
    nvim_lang_dictionary::NvimLanguageReadonlyDictionary,
    nvim_language::file::NvimLanguageFile,
};

#[derive(Debug)]
pub struct NvimLanguageCore {
    lang_tool_client: Arc<LangToolClient>,
}

impl NvimLanguageCore {
    pub fn new(lang_tool_url: Option<String>, lang: Option<String>) -> NvimLanguageCore {
        return NvimLanguageCore {
            lang_tool_client: Arc::new(LangToolClient::new(lang_tool_url, lang)),
        };
    }

    pub fn get_language_tool_client(&self) -> Arc<LangToolClient> {
        return self.lang_tool_client.clone();
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

    pub fn process_file(
        &self,
        file_path: String,
        nvim_language_readonly_dictionary: NvimLanguageReadonlyDictionary,
    ) -> NvimLanguageFile {
        if file_path.is_empty() {
            warn!("No file path was provided");
            return NvimLanguageFile::empty();
        }

        let nvim_language_readonly_dictionary = Arc::new(nvim_language_readonly_dictionary);

        let languagetool_client = self.get_language_tool_client();
        let local_client = languagetool_client.clone();
        let runtime = local_client.get_runtime();

        return runtime.block_on(async {
            let code_file =
                match CodeFile::new(file_path.clone(), nvim_language_readonly_dictionary.clone())
                    .await
                {
                    Some(code_file) => code_file,
                    None => {
                        warn!(
                            "nvim-lang-core does not support this file type: {}",
                            file_path
                        );
                        return NvimLanguageFile::empty();
                    }
                };

            let language_tool_file = LanguageToolFile::new(code_file, languagetool_client).await;

            return NvimLanguageFile::new(language_tool_file, nvim_language_readonly_dictionary)
                .await;
        });
    }

    fn get_programming_file(&self, file_path: &String) -> Option<ProgrammingLanguageType> {
        if file_path.ends_with(&RUST.extension) {
            return Some(ProgrammingLanguageType::Rust);
        }

        if file_path.ends_with(&LUA.extension) {
            return Some(ProgrammingLanguageType::Lua);
        }
        return None;
    }
}
