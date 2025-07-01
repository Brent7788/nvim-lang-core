use log::debug;
use nvim_lang_core::common::logger::Logger;
use std::env;

use nvim_lang_core::common::test::ExpectedTrait;
use nvim_lang_core::{
    common::test::{get_project_path, Expected},
    nvim_lang_core::NvimLangCore,
};
use nvim_lang_core::{
    nvim_lang_dictionary::NvimLanguageDictionary, nvim_language::core::NvimLanguageCore,
};
use rstest::rstest;

#[rstest]
#[case("/rust/strings/simple_string.rs", 
vec![
        Expected::new(2, 17, 24, 5, "brances", vec!["branches"]),
        Expected::new(2, 41, 50, 2, "especialy", vec!["especially"]),
        Expected::new(2, 82, 88, 2, "prduct", vec!["product"]),
        Expected::new(2, 5, 6, 1, "a", vec!["A"])
    ]
)]
#[case("/lua/strings/simple_string.lua", 
vec![
        Expected::new(2, 17, 24, 5, "brances", vec!["branches"]),
        Expected::new(2, 41, 50, 2, "especialy", vec!["especially"]),
        Expected::new(2, 82, 88, 2, "prduct", vec!["product"]),
        Expected::new(2, 5, 6, 1, "a", vec!["A"])
    ]
)]
fn simple_string_should_be(#[case] path: &str, #[case] mut expected: Vec<Expected>) {
    env::set_var("RUST_BACKTRACE", "1");

    let file_path = get_project_path(path);

    let nvim_language_dictionary = NvimLanguageDictionary::new(true);
    let core = NvimLanguageCore::new(None, None);

    let mut result = core.process_file(file_path, nvim_language_dictionary.to_readonly());
    expected.expected_sorting_order();
    result.expected_sorting_order();

    Expected::data_len_to_be(4, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
}

#[rstest]
#[case("/rust/strings/multiple_strings.rs",
vec![
Expected::new(2, 8, 14, 2, "prduct", vec!["product"]),
Expected::new(2, 20, 29, 1, "oparation", vec!["operation"]),
Expected::new(2, 35, 41, 2, "purson", vec!["person", "parson"]),
Expected::new(3, 21, 30, 1, "OPARATION", vec!["OPERATION"]),
Expected::new(3, 31, 37, 1, "PRDUCT", vec!["PRODUCT"])
    ])]
#[case("/lua/strings/multiple_strings.lua",
vec![
Expected::new(2, 8, 14, 2, "prduct", vec!["product"]),
Expected::new(2, 20, 29, 1, "oparation", vec!["operation"]),
Expected::new(2, 35, 41, 2, "purson", vec!["person", "parson"]),
Expected::new(3, 23, 32, 1, "OPARATION", vec!["OPERATION"]),
Expected::new(3, 33, 39, 1, "PRDUCT", vec!["PRODUCT"])
    ]
)]
fn multiple_strings_should_be(#[case] path: &str, #[case] mut expected: Vec<Expected>) {
    env::set_var("RUST_BACKTRACE", "1");

    // Logger::console_init();
    let file_path = get_project_path(path);

    let nvim_language_dictionary = NvimLanguageDictionary::new(true);
    let core = NvimLanguageCore::new(None, None);

    let mut result = core.process_file(file_path, nvim_language_dictionary.to_readonly());

    expected.expected_sorting_order();
    result.expected_sorting_order();
    // debug!("{:#?}", result);
    // log::logger().flush();
    Expected::data_len_to_be(5, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
}
