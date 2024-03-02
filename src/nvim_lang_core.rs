use log::{debug, warn};
use nvim_oxi::Result;

use crate::lang_tool::LanguageToolFile;
use crate::lang_tool_client::LangToolClient;
use crate::nvim_lang::NvimLanguageFile;
use crate::programming_lang::{ProgrammingFile, ProgrammingLanguage};

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

    // TODO: Find better method name.
    pub async fn process_file(&self, file_path: String) -> Result<NvimLanguageFile> {
        if file_path.is_empty() {
            warn!("No file path was provided");
            return Ok(NvimLanguageFile::new());
        }

        let lang = match self.get_file_type(&file_path) {
            Some(lang) => lang,
            None => {
                warn!(
                    "nvim-lang-core does not support this file type: {}",
                    file_path
                );
                return Ok(NvimLanguageFile::new());
            }
        };

        let prog_file = ProgrammingFile::create(&file_path, &lang);

        let lang_tool_file = LanguageToolFile::new(&prog_file, &self.lang_tool_client).await;

        // debug!("LANG FILE: {:#?}", lang_tool_file.code);

        return Ok(NvimLanguageFile::create(&lang_tool_file));
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
