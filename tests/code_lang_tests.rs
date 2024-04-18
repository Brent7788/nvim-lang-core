use log::info;
use nvim_lang_core::common::logger::Logger;
use nvim_lang_core::common::test::get_project_path;
use nvim_lang_core::common::test::Expected;
use nvim_lang_core::nvim_lang_core::NvimLangCore;
use rstest::rstest;
use std::env;

#[rstest]
#[case("/rust/codes/simple_code.rs", vec![
    Expected::new(1, 7, 15, 1, "upercase", vec!["uppercase"]),
    Expected::new(1, 16, 22, 2, "prduct", vec!["product", "pr duct"])
])]
#[case("/lua/codes/simple_code.lua", vec![
    Expected::new(2, 4, 12, 1, "upercase", vec!["uppercase"]),
    Expected::new(2, 24, 30, 2, "prduct", vec!["product", "pr duct"])
])]
fn simple_code_should_be(#[case] path: &str, #[case] expected: Vec<Expected>) {
    env::set_var("RUST_BACKTRACE", "1");

    let file_path = get_project_path(path);

    let core = NvimLangCore::new(None, None);

    let result = core.process_file(file_path, None);

    Expected::data_len_to_be(2, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
}

#[rstest]
#[case("/rust/codes/multiple_code.rs", 12,
    vec![
    Expected::new(2, 15, 20, 3, "Foldr", vec!["Fold", "Folder", "Folds"]),
    Expected::new(6, 9, 14, 3, "Foldr", vec!["Fold", "Folder", "Folds"]),
    Expected::new(7, 62, 67, 3, "Foldr", vec!["Fold", "Folder", "Folds"]),
    Expected::new(12, 19, 24, 3, "Foldr", vec!["Fold", "Folder", "Folds"]),
    Expected::new(7, 11, 18, 2, "generte", vec!["generate", "gene rte"]),
    Expected::new(3, 4, 9, 3, "foldr", vec!["fold", "folder", "folds"]),
    Expected::new(7, 19, 24, 3, "foldr", vec!["fold", "folder", "folds"]),
    Expected::new(12, 27, 32, 3, "foldr", vec!["fold", "folder", "folds"]),
    Expected::new(7, 25, 31, 1, "systim", vec!["system"]),
    Expected::new(12, 39, 45, 1, "systim", vec!["system"]),
    Expected::new(
        7,
        41,
        48,
        18,
        "procces",
        vec!["process", "produces", "prices"],
    ),
    Expected::new(
        8,
        11,
        18,
        18,
        "procces",
        vec!["process", "produces", "prices"],
    )
])]
#[case("/lua/codes/multiple_code.lua", 12, 
    vec![
    Expected::new(1, 10, 15, 3, "Foldr", vec!["Fold", "Folder", "Folds"]),
    Expected::new(5, 13, 18, 3, "Foldr", vec!["Fold", "Folder", "Folds"]),
    Expected::new(15, 11, 16, 3, "Foldr", vec!["Fold", "Folder", "Folds"]),
    Expected::new(5, 19, 26, 2, "generte", vec!["generate", "gene rte"]),
    Expected::new(2, 4, 9, 3, "foldr", vec!["fold", "folder", "folds"]),
    Expected::new(5, 27, 32, 3, "foldr", vec!["fold", "folder", "folds"]),
    Expected::new(11, 8, 13, 3, "foldr", vec!["fold", "folder", "folds"]),
    Expected::new(5, 33, 39, 1, "systim", vec!["system"]),
    Expected::new(11, 21, 27, 1, "systim", vec!["system"]),
    Expected::new(
        5,
        41,
        48,
        18,
        "procces",
        vec!["process", "produces", "prices"],
    ),
    Expected::new(
        6,
        7,
        14,
        18,
        "procces",
        vec!["process", "produces", "prices"],
    )
])]
fn multiple_code_should_be(
    #[case] path: &str,
    #[case] data_len: usize,
    #[case] expected: Vec<Expected>,
) {
    env::set_var("RUST_BACKTRACE", "1");

    let file_path = get_project_path(path);

    let core = NvimLangCore::new(None, None);

    let result = core.process_file(file_path, None);

    Expected::data_len_to_be(data_len, &result);
    for (index, exp) in expected.iter().enumerate() {
        exp.assert(index, &result)
    }
}
