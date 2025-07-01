mod code_file_tests;
use std::env;
use std::sync::Arc;

use log::{debug, info};
use nvim_lang_core::common::test::ExpectedTrait;
use nvim_lang_core::nvim_lang_dictionary::NvimLanguageDictionary;
use nvim_lang_core::nvim_language::file::NvimLanguageFile;
use nvim_lang_core::{
    common::{
        logger::Logger,
        test::{get_project_path, Expected},
    },
    nvim_lang_core::NvimLangCore,
    nvim_language::core::NvimLanguageCore,
};
use tokio::runtime::Runtime;

use rstest::rstest;

#[rstest]
#[case("/rust/comments/simple_one_line_comment.rs", vec![
    Expected::new(1, 10, 15, 6, "simle", vec!["simple", "smile", "simile"])
])]
#[case("/lua/comments/simple_one_line_comment.lua", vec![
    Expected::new(1, 10, 15, 6, "simle", vec!["simple", "smile", "simile"])
])]
fn simple_code_should_be(#[case] path: &str, #[case] expected: Vec<Expected>) {
    // env::set_var("RUST_BACKTRACE", "1");

    // Logger::console_init();
    let file_path = get_project_path(path);
    let nvim_language_dictionary = NvimLanguageDictionary::new(true);
    let core = NvimLanguageCore::new(None, None);

    let result = core.process_file(file_path, nvim_language_dictionary.to_readonly());

    Expected::data_len_to_be(1, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
    // log::logger().flush();
}

#[rstest]
#[case("/rust/comments/multiple_comments.rs", vec![
    Expected::new(1, 16, 26, 1, "commmented", vec!["commented"]),
    Expected::new(2, 21, 29, 2, "invoving", vec!["involving", "invoking"]),
    Expected::new(4, 2, 3, 1, "a", vec!["A"]),
    Expected::new(4, 14, 21, 5, "brances", vec!["branches"]),
    Expected::new(4, 38, 47, 2, "especialy", vec!["especially"]),
    Expected::new(4, 79, 85, 2, "prduct", vec!["product"])
])]
#[case("/lua/comments/multiple_comments.lua", vec![
    Expected::new(1, 16, 26, 1, "commmented", vec!["commented"]),
    Expected::new(2, 21, 29, 2, "invoving", vec!["involving", "invoking"]),
    Expected::new(4, 2, 3, 1, "a", vec!["A"]),
    Expected::new(4, 14, 21, 5, "brances", vec!["branches"]),
    Expected::new(4, 38, 47, 2, "especialy", vec!["especially"]),
    Expected::new(4, 79, 85, 2, "prduct", vec!["product"])
])]
fn multiple_comment_should_be(#[case] path: &str, #[case] mut expected: Vec<Expected>) {
    env::set_var("RUST_BACKTRACE", "1");

    let file_path = get_project_path(path);

    let nvim_language_dictionary = NvimLanguageDictionary::new(true);
    let core = NvimLanguageCore::new(None, None);

    let mut result = core.process_file(file_path, nvim_language_dictionary.to_readonly());

    expected.expected_sorting_order();
    result.expected_sorting_order();

    Expected::data_len_to_be(6, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
}

#[rstest]
#[case("/rust/comments/comment_block.rs", vec![
    Expected::new(1, 17, 27, 1, "commmented", vec!["commented"]),
    Expected::new(2, 19, 27, 2, "invoving", vec!["involving", "invoking"]),
    Expected::new(4, 0, 1, 1, "a", vec!["A"]),
    Expected::new(4, 12, 19, 5, "brances", vec!["branches"]),
    Expected::new(4, 36, 45, 2, "especialy", vec!["especially"]),
    Expected::new(4, 77, 83, 2, "prduct", vec!["product"]),
])]
#[case("/lua/comments/comment_block.lua", vec![
    Expected::new(1, 19, 29, 1, "commmented", vec!["commented"]),
    Expected::new(2, 19, 27, 2, "invoving", vec!["involving", "invoking"]),
    Expected::new(4, 0, 1, 1, "a", vec!["A"]),
    Expected::new(4, 12, 19, 5, "brances", vec!["branches"]),
    Expected::new(4, 36, 45, 2, "especialy", vec!["especially"]),
    Expected::new(4, 77, 83, 2, "prduct", vec!["product"]),
])]
fn ccomment_block_should_be(#[case] path: &str, #[case] mut expected: Vec<Expected>) {
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
    Expected::data_len_to_be(6, &result);
    // Expected::data_len_to_be(8, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
}

#[rstest]
#[case("/rust/comments/full_comment.rs", vec![
    Expected::new(3, 34, 42, 1, "too have", vec!["to have"]),
    Expected::new(4, 43, 50, 1, "colours", vec!["colors"]),
    Expected::new(4, 65, 73, 2, "seplling", vec!["selling", "spelling"]),
    Expected::new(4, 90, 100, 1, "underilnes", vec!["underlines"]),
    Expected::new(5, 4, 15, 1, "Furthermore", vec!["Furthermore,"]),
    Expected::new(5, 24, 31, 1, "error's", vec!["errors"]),
    Expected::new(6, 41, 61, 1, "in a reliable manner", vec!["reliably"]),
    Expected::new(7, 4, 7, 1, "did", vec!["Did"]),
    Expected::new(7, 30, 33, 1, "sea", vec!["see"]),
    Expected::new(7, 46, 61, 1, "double clicking", vec!["double-clicking"]),
    Expected::new(7, 70, 73, 1, "Its", vec!["It's"]),
    Expected::new(7, 74, 75, 1, "a", vec!["an"]),
    Expected::new(8, 33, 37, 1, "youd", vec!["you'd"]),
    Expected::new(8, 68, 78, 1, "over sea's", vec!["overseas"]),
    Expected::new(9, 18, 37, 1, "PM in the afternoon", vec!["PM"]),
    Expected::new(9, 41, 51, 0, "Monday, 27", vec!["Sunday, 27", "Monday, 28"])
])]
#[case("/lua/comments/full_comment.lua", vec![
    Expected::new(3, 34, 42, 1, "too have", vec!["to have"]),
    Expected::new(4, 43, 50, 1, "colours", vec!["colors"]),
    Expected::new(4, 65, 73, 2, "seplling", vec!["selling", "spelling"]),
    Expected::new(4, 90, 100, 1, "underilnes", vec!["underlines"]),
    Expected::new(5, 4, 15, 1, "Furthermore", vec!["Furthermore,"]),
    Expected::new(5, 24, 31, 1, "error's", vec!["errors"]),
    Expected::new(6, 41, 61, 1, "in a reliable manner", vec!["reliably"]),
    Expected::new(7, 4, 7, 1, "did", vec!["Did"]),
    Expected::new(7, 30, 33, 1, "sea", vec!["see"]),
    Expected::new(7, 46, 61, 1, "double clicking", vec!["double-clicking"]),
    Expected::new(7, 70, 73, 1, "Its", vec!["It's"]),
    Expected::new(7, 74, 75, 1, "a", vec!["an"]),
    Expected::new(8, 33, 37, 1, "youd", vec!["you'd"]),
    Expected::new(8, 68, 78, 1, "over sea's", vec!["overseas"]),
    Expected::new(9, 18, 37, 1, "PM in the afternoon", vec!["PM"]),
    Expected::new(9, 41, 51, 0, "Monday, 27", vec!["Sunday, 27", "Monday, 28"])
])]
fn full_comment_should_be(#[case] path: &str, #[case] mut expected: Vec<Expected>) {
    env::set_var("RUST_BACKTRACE", "1");

    // Logger::console_init();
    let file_path = get_project_path(path);

    let nvim_language_dictionary = NvimLanguageDictionary::new(true);
    let core = NvimLanguageCore::new(None, None);

    core.get_language_tool_client().docker_setup();

    let mut result = core.process_file(file_path, nvim_language_dictionary.to_readonly());
    expected.expected_sorting_order();
    result.expected_sorting_order();
    // debug!("{:#?}", result);
    // log::logger().flush();
    Expected::data_len_to_be(16, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
}
