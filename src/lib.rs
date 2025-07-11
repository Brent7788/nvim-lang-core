use std::sync::{Arc, Mutex};

use common::logger::Logger;
use log::{error, info};
use nvim_oxi::{Dictionary, Function, Object, Result};

use crate::{
    nvim_lang_dictionary::{NvimLanguageDictionary, NvimLanguageReadonlyDictionary},
    nvim_language::{core::NvimLanguageCore, file::NvimLanguageFile},
};

pub mod code;
pub mod common;
pub mod lang_tool;
pub mod lang_tool_client;
pub mod language_tool;
pub mod modules;
pub mod nvim_lang;
pub mod nvim_lang_core;
pub mod nvim_lang_dictionary;
pub mod nvim_language;
pub mod programming_lang;

#[nvim_oxi::module]
fn main() -> Result<Dictionary> {
    Logger::file_init(None);

    info!("Nvim Language Core Starting...");

    let nvim_lang_core = Arc::new(NvimLanguageCore::new(None, None));
    let nvim_lang_core_start_processing = nvim_lang_core.clone();
    let nvim_lang_file: Arc<Mutex<Option<NvimLanguageFile>>> = Arc::new(Mutex::new(None));
    let nvim_language_dictionary = Arc::new(Mutex::new(NvimLanguageDictionary::new(false)));
    let nvim_language_dictionary_start_processing = nvim_language_dictionary.clone();
    let nvim_lang_file_cp = nvim_lang_file.clone();

    let start_processing_fn = move |file_path: String| {
        info!("Start Processing file {file_path}");

        log::logger().flush();
        let nvim_language_dictionary = nvim_language_dictionary_start_processing.clone();
        let nvim_lang_file = nvim_lang_file.clone();
        let nvim_lang_core = nvim_lang_core_start_processing.clone();

        nvim_lang_core_start_processing.spawn_blocking(move || {
            let nvim_language_readonly_dictionary = match nvim_language_dictionary.lock() {
                Ok(guard) => guard.to_readonly(),
                Err(e) => {
                    error!(
                        "Error locking the nvim language dictionary in remove word! {:#?}",
                        e
                    );
                    NvimLanguageReadonlyDictionary::new()
                }
            };

            let nvim_lang_file_p =
                nvim_lang_core.process_file(file_path.clone(), nvim_language_readonly_dictionary);

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

            info!("Done Processing file {}", file_path);
            match *nvim_lang_file_gard {
                Some(_) => return,
                None => {
                    *nvim_lang_file_gard = Some(nvim_lang_file_p);
                }
            };
        });

        return Result::Ok(());
    };

    let check_process_fn = move |()| {
        log::logger().flush();

        let mut nvim_lang_file_guard = match nvim_lang_file_cp.try_lock() {
            Ok(l) => l,
            Err(e) => {
                info!("nvim language file is busy processing {:#?}", e);
                log::logger().flush();
                return Result::Ok(None);
            }
        };

        let nvim_lang_file_dest =
            std::mem::replace::<Option<NvimLanguageFile>>(&mut *nvim_lang_file_guard, None);

        return Result::Ok(nvim_lang_file_dest);
    };

    let nvim_lang_core_docker_setup = nvim_lang_core.clone();
    let languagetool_docker_setup_fn = move |()| {
        let nvim_lang_core = nvim_lang_core_docker_setup.clone();

        nvim_lang_core_docker_setup.spawn_blocking(move || {
            nvim_lang_core.get_language_tool_client().docker_setup();

            log::logger().flush();
        });

        return Result::Ok(());
    };

    let nvim_lang_core_add_word = nvim_lang_core.clone();
    let nvim_language_dictionary_add_word = nvim_language_dictionary.clone();

    let add_word_fn = move |word: String| {
        let nvim_lang_dictionary = nvim_language_dictionary_add_word.clone();

        nvim_lang_core_add_word.spawn_blocking(move || {
            let mut nvim_language_dictionary_gard = match nvim_lang_dictionary.lock() {
                Ok(gard) => gard,
                Err(e) => {
                    error!(
                        "Error locking the nvim language dictionary in add word! {:#?}",
                        e
                    );
                    return;
                }
            };

            nvim_language_dictionary_gard.append_word(word);
        });

        return Result::Ok(());
    };

    let nvim_lang_core_remove_word = nvim_lang_core.clone();
    let nvim_language_dictionary_remove_word = nvim_language_dictionary.clone();

    let remove_word_fn = move |word: String| {
        let nvim_lang_dictionary = nvim_language_dictionary_remove_word.clone();

        nvim_lang_core_remove_word.spawn_blocking(move || {
            let mut nvim_language_dictionary_gard = match nvim_lang_dictionary.lock() {
                Ok(gard) => gard,
                Err(e) => {
                    error!(
                        "Error locking the nvim language dictionary in remove word! {:#?}",
                        e
                    );
                    return;
                }
            };

            nvim_language_dictionary_gard.remove_word(word);
        });

        return Result::Ok(());
    };

    let nvim_language_dictionary_get_words = nvim_language_dictionary.clone();

    let get_words_fn = move |()| {
        let nvim_language_dictionary_gard = match nvim_language_dictionary_get_words.lock() {
            Ok(gard) => gard,
            Err(e) => {
                error!(
                    "Error locking the nvim language dictionary in remove word! {:#?}",
                    e
                );
                return Result::Ok(Vec::new());
            }
        };

        return Result::Ok(nvim_language_dictionary_gard.get_words());
    };

    let nvim_lang_core_does_support_language = nvim_lang_core.clone();

    let does_support_language_fn = move |file_path: String| {
        if file_path.is_empty() {
            return Result::Ok(false);
        }

        if NvimLanguageCore::support_file(&file_path) {
            return Result::Ok(true);
        }

        return Result::Ok(false);
    };

    info!("Nvim Language Core has Started");

    log::logger().flush();

    let start_processing_fn = Function::from_fn(start_processing_fn);
    let check_process_fn = Function::from_fn(check_process_fn);
    let languagetool_docker_setup_fn = Function::from_fn(languagetool_docker_setup_fn);
    let add_word_fn = Function::from_fn(add_word_fn);
    let remove_word_fn = Function::from_fn(remove_word_fn);
    let get_words_fn = Function::from_fn(get_words_fn);
    let does_support_language_fn = Function::from_fn(does_support_language_fn);

    return Ok(Dictionary::from_iter([
        ("start_processing", Object::from(start_processing_fn)),
        ("check_process", Object::from(check_process_fn)),
        (
            "languagetool_docker_setup",
            Object::from(languagetool_docker_setup_fn),
        ),
        ("add_word", Object::from(add_word_fn)),
        ("remove_word", Object::from(remove_word_fn)),
        ("get_words", Object::from(get_words_fn)),
        (
            "does_support_language",
            Object::from(does_support_language_fn),
        ),
    ]));
}
