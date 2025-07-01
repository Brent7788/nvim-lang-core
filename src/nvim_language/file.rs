use std::{collections::HashSet, sync::Arc};

use log::error;
use nvim_oxi::{
    conversion::{FromObject, ToObject},
    lua,
    serde::{Deserializer, Serializer},
    Object,
};
use serde::{Deserialize, Serialize};
use tokio::spawn;

use crate::{
    common::test::ExpectedTrait, language_tool::language_tool_file::LanguageToolFile,
    nvim_lang_dictionary::NvimLanguageReadonlyDictionary, nvim_language::line::NvimLanguageLine,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NvimLanguageFile {
    pub nvim_lang_lines: Vec<NvimLanguageLine>,
}

impl NvimLanguageFile {
    pub async fn new(
        language_tool_file: LanguageToolFile,
        language_dictionary: Arc<NvimLanguageReadonlyDictionary>,
    ) -> Self {
        let mut nvim_language_lines: HashSet<NvimLanguageLine> = HashSet::new();
        let mut nvim_language_line_handles = Vec::new();

        for language_tool_lines in language_tool_file.lines {
            let language_tool_line_type = Arc::new(language_tool_lines.lines);
            let matches = language_tool_lines.lang_tool_response.matches;

            for nvim_match in matches {
                let language_tool_line_type = language_tool_line_type.clone();
                let language_dictionary = language_dictionary.clone();
                nvim_language_line_handles.push(spawn(NvimLanguageLine::new_lines(
                    nvim_match,
                    language_tool_line_type.clone(),
                    language_dictionary,
                )));
            }
        }

        for handle in nvim_language_line_handles {
            match handle.await {
                Ok(nvim_lang_lines) => match nvim_lang_lines {
                    Some(nvim_lang_lines) => nvim_language_lines.extend(nvim_lang_lines),
                    None => {}
                },
                Err(e) => error!("NvimLanguageFile::new error: {:?}", e),
            }
        }

        return NvimLanguageFile {
            nvim_lang_lines: nvim_language_lines.into_iter().collect(),
        };
    }

    pub fn empty() -> Self {
        return NvimLanguageFile {
            nvim_lang_lines: Vec::new(),
        };
    }
}

impl ExpectedTrait for NvimLanguageFile {
    fn expected_sorting_order(&mut self) {
        self.nvim_lang_lines.sort_by(|l, b| {
            l.line_number
                .cmp(&b.line_number)
                .then_with(|| l.start_column.cmp(&b.start_column))
        });
    }
}

impl FromObject for NvimLanguageFile {
    fn from_object(object: nvim_oxi::Object) -> Result<Self, nvim_oxi::conversion::Error> {
        return Self::deserialize(Deserializer::new(object)).map_err(Into::into);
    }
}

impl ToObject for NvimLanguageFile {
    fn to_object(self) -> Result<nvim_oxi::Object, nvim_oxi::conversion::Error> {
        return self.serialize(Serializer::new()).map_err(Into::into);
    }
}

impl lua::Poppable for NvimLanguageFile {
    unsafe fn pop(lstate: *mut lua::ffi::lua_State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for NvimLanguageFile {
    unsafe fn push(
        self,
        lstate: *mut nvim_oxi::lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}
