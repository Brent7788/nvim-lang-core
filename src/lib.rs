use common::logger::Logger;
use log::info;
use nvim_oxi::{Dictionary, Function, Object, Result};

use crate::nvim_lang_core::NvimLangCore;

pub mod common;
pub mod lang_tool;
pub mod lang_tool_client;
pub mod modules;
pub mod nvim_lang;
pub mod nvim_lang_core;
pub mod programming_lang;

#[nvim_oxi::module]
fn main() -> Result<Dictionary> {
    // TODO: Remember to flush the logs
    Logger::file_init(None);

    info!("Nvim Language Core Starting...");
    info!("Starting Up Tokio Runtime...");

    let nvim_lang_core = NvimLangCore::new(None, None);

    let process_file = move |file_path: String| {
        info!("Process file {file_path}");
        return nvim_lang_core.process_file(file_path);
    };

    let pr = Function::from_fn_once(process_file);

    return Ok(Dictionary::from_iter([("process", Object::from(pr))]));
}
