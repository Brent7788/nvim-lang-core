use std::sync::{Arc, Mutex};

use common::logger::Logger;
use log::{error, info};
use nvim_oxi::{Dictionary, Function, Object, Result};

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

    let nvim_lang_core = Arc::new(NvimLangCore::new(None, None));
    let nvim_lang_file: Arc<Mutex<Option<NvimLanguageFile>>> = Arc::new(Mutex::new(None));
    let nvim_lang_file_cp = nvim_lang_file.clone();

    let start_processing_fn = move |file_path: String| {
        info!("Start Processing file {file_path}");

        log::logger().flush();

        let tokio_runtime = nvim_lang_core
            .lang_tool_client
            .tokio_runtime
            .as_ref()
            .unwrap();

        let nvim_lang_file = nvim_lang_file.clone();
        let nvim_lang_core = nvim_lang_core.clone();

        tokio_runtime.spawn_blocking(move || {
            let nvim_lang_file_p = nvim_lang_core.process_file(file_path);

            let mut nvim_lang_file_gard = match nvim_lang_file.lock() {
                Ok(l) => l,
                Err(e) => {
                    error!(
                        "Error locking the nvim language file in start processing! {:#?}",
                        e
                    );
                    return;
                }
            };

            info!("Done Processing file {}", nvim_lang_file_p.file_path);
            match *nvim_lang_file_gard {
                Some(_) => return,
                None => {
                    *nvim_lang_file_gard = Some(nvim_lang_file_p);
                }
            };
        });

        return Result::Ok(());
    };

    let check_process = move |()| {
        log::logger().flush();

        let mut nvim_lang_file_gard = match nvim_lang_file_cp.try_lock() {
            Ok(l) => l,
            Err(e) => {
                info!("nvim language file is busy processing {:#?}", e);
                log::logger().flush();
                return Result::Ok(None);
            }
        };

        let nvim_lang_file_dest =
            std::mem::replace::<Option<NvimLanguageFile>>(&mut *nvim_lang_file_gard, None);

        return Result::Ok(nvim_lang_file_dest);
    };

    info!("Nvim Language Core has Started");

    log::logger().flush();

    let pr = Function::from_fn(start_processing_fn);
    let cpr = Function::from_fn(check_process);

    return Ok(Dictionary::from_iter([
        ("start_processing", Object::from(pr)),
        ("check_process", Object::from(cpr)),
    ]));
}
