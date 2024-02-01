use log::{info, warn};

use crate::lang_tool::LangToolCore;
use crate::lang_tool_client::LangToolClient;
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

    pub async fn process_file(&self, file_path: String) {
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

        //info!("{:#?}", prog_file);
        // prog_file.debug_p();

        let lang_tool_core = LangToolCore::new(&prog_file, &self.lang_tool_client).await;
        info!("{:#?}", lang_tool_core);

        let n = lang_tool_core.get_data();
        info!("{:#?}", n);
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
