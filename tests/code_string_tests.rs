use std::env;

use nvim_lang_core::{
    common::test::{get_project_path, Expected},
    nvim_lang_core::NvimLangCore,
};
use nvim_lang_core::{
    nvim_lang_dictionary::NvimLanguageDictionary, nvim_language::core::NvimLanguageCore,
};
use rstest::rstest;

#[rstest]
#[case("/rust/strings/simple_string.rs")]
#[case("/lua/strings/simple_string.lua")]
fn simple_string_should_be(#[case] path: &str) {
    env::set_var("RUST_BACKTRACE", "1");

    let file_path = get_project_path(path);

    let nvim_language_dictionary = NvimLanguageDictionary::new(true);
    let core = NvimLanguageCore::new(None, None);

    let result = core.process_file(file_path, nvim_language_dictionary.to_readonly());

    Expected::data_len_to_be(4, &result);
    Expected::new(2, 17, 24, 5, "brances", vec!["branches"]).assert(0, &result);
    Expected::new(2, 41, 50, 2, "especialy", vec!["especially"]).assert(1, &result);
    Expected::new(2, 82, 88, 2, "prduct", vec!["product"]).assert(2, &result);
    Expected::new(2, 5, 6, 1, "a", vec!["A"]).assert(3, &result);
}

#[rstest]
#[case("/rust/strings/multiple_strings.rs")]
#[case("/lua/strings/multiple_strings.lua")]
fn multiple_strings_should_be(#[case] path: &str) {
    env::set_var("RUST_BACKTRACE", "1");

    let file_path = get_project_path(path);

    let nvim_language_dictionary = NvimLanguageDictionary::new(true);
    let core = NvimLanguageCore::new(None, None);

    let result = core.process_file(file_path, nvim_language_dictionary.to_readonly());

    Expected::data_len_to_be(5, &result);
    Expected::new(2, 8, 14, 2, "prduct", vec!["product", "pr duct"]).assert(0, &result);
    Expected::new(2, 20, 29, 1, "oparation", vec!["operation"]).assert(1, &result);
    Expected::new(2, 35, 41, 2, "purson", vec!["person", "parson"]).assert(2, &result);
    Expected::new(3, 21, 30, 1, "OPARATION", vec!["OPERATION"]).assert(3, &result);
    Expected::new(3, 31, 37, 1, "PRDUCT", vec!["PRODUCT"]).assert(4, &result);
}
