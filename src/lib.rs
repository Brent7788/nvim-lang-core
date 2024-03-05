use core::panic;

use common::logger::Logger;
use log::{error, info};
use nvim_oxi::{Dictionary, Function, Object, Result};
use tokio::{runtime::Runtime, task::JoinHandle};

use crate::{nvim_lang::NvimLanguageFile, nvim_lang_core::NvimLangCore};

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

    let tokio_runtime = Runtime::new();

    let tokio_runtime = match tokio_runtime {
        Ok(tokio_runtime) => tokio_runtime,
        Err(e) => {
            error!("Unable to start up Tokio Runtime {:#?}", e);
            panic!(); // TODO: This should not panic
        }
    };

    let nvim_lang_core = NvimLangCore::new(None, None);

    let process_file = move |file_path: String| {
        info!("Process file {file_path}");
        //
        // let handler: JoinHandle<Result<NvimLanguageFile>> = tokio_runtime.spawn(async move {
        //     return nvim_lang_core.process_file(file_path).await;
        // });
        //
        // let nvim_lang_file = tokio_runtime
        //     .block_on(handler)
        //     .expect("Something went wrong!");
        // //
        // return nvim_lang_file;
        return Result::Ok(NvimLanguageFile::new());
    };

    let pr = Function::from_fn_once(process_file);

    return Ok(Dictionary::from_iter([("process", Object::from(pr))]));
}

// fn main() {}
