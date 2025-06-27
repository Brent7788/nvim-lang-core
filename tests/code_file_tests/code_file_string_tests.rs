use log::info;
use rstest::rstest;
use std::env;
use tokio::runtime::{self, Runtime};

use nvim_lang_core::nvim_lang_dictionary::NvimLanguageDictionary;
use nvim_lang_core::{
    code::{
        code_file::{CodeFile, CodeType},
        programming::{ProgrammingLanguage, ProgrammingLanguageType, LUA, RUST},
    },
    common::{logger::Logger, test::get_project_path},
};

#[rstest]
#[case("/rust/strings/simple_string.rs", ProgrammingLanguageType::Rust)]
#[case("/lua/strings/simple_string.lua", ProgrammingLanguageType::Lua)]
fn simple_string_should_be(#[case] path: &str, #[case] lang_type: ProgrammingLanguageType) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let nvim_language_dictionary = NvimLanguageDictionary::new(true);
        match lang_type {
            ProgrammingLanguageType::Lua => assert_code_file(CodeFile::create(&file_path, &LUA, nvim_language_dictionary.to_readonly()).await),
            ProgrammingLanguageType::Rust => assert_code_file(CodeFile::create(&file_path, &RUST, nvim_language_dictionary.to_readonly()).await),
        }

        fn assert_code_file<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
            code_file: CodeFile<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
        ) {
            assert_eq!(2, code_file.lines.len());
            let line = &code_file.lines[0];
            assert_ne!(0, line.hash);
            assert_eq!(1, line.line.line_number);
            assert_eq!("TEXT", line.value);
            // assert_eq!(data.3, line.line.original_line);
            assert_eq!(true, matches!(line.tp, CodeType::Code));
            let line = &code_file.lines[1];
            assert_ne!(0, line.hash);
            assert_eq!(2, line.line.line_number);
            assert_eq!("a shop with brances in many places, especialy one selling a specific type of prduct.", line.value);
            // assert_eq!(data.3, line.line.original_line);
            assert_eq!(true, matches!(line.tp, CodeType::String));
        }
    });

    // log::logger().flush();
}

#[rstest]
#[case("/rust/strings/multiple_strings.rs", ProgrammingLanguageType::Rust)]
#[case("/lua/strings/multiple_strings.lua", ProgrammingLanguageType::Lua)]
fn multiple_string_should_be(#[case] path: &str, #[case] lang_type: ProgrammingLanguageType) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let nvim_language_dictionary = NvimLanguageDictionary::new(true);
        match lang_type {
            ProgrammingLanguageType::Lua => assert_code_file(
                CodeFile::create(&file_path, &LUA, nvim_language_dictionary.to_readonly()).await,
            ),
            ProgrammingLanguageType::Rust => assert_code_file(
                CodeFile::create(&file_path, &RUST, nvim_language_dictionary.to_readonly()).await,
            ),
        }

        fn assert_code_file<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
            code_file: CodeFile<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
        ) {
            assert_eq!(7, code_file.lines.len());
            let line = &code_file.lines[0];
            assert_ne!(0, line.hash);
            assert_eq!(1, line.line.line_number);
            assert_eq!("system system", line.value);
            // assert_eq!(data.3, line.line.original_line);
            assert_eq!(true, matches!(line.tp, CodeType::Code));
            let line = &code_file.lines[1];
            assert_ne!(0, line.hash);
            assert_eq!(2, line.line.line_number);
            assert_eq!("prduct", line.value);
            assert_eq!(true, matches!(line.tp, CodeType::String));
            let line = &code_file.lines[2];
            assert_ne!(0, line.hash);
            assert_eq!(2, line.line.line_number);
            assert_eq!("oparation", line.value);
            assert_eq!(true, matches!(line.tp, CodeType::String));
            let line = &code_file.lines[3];
            assert_ne!(0, line.hash);
            assert_eq!(2, line.line.line_number);
            assert_eq!("purson", line.value);
            assert_eq!(true, matches!(line.tp, CodeType::String));
            let line = &code_file.lines[4];
            assert_ne!(0, line.hash);
            assert_eq!(2, line.line.line_number);
            assert_eq!("system", line.value);
            assert_eq!(true, matches!(line.tp, CodeType::String));
            let line = &code_file.lines[5];
            assert_ne!(0, line.hash);
            assert_eq!(3, line.line.line_number);
            assert_eq!("OPARATION_PRDUCT", line.value);
            assert_eq!(true, matches!(line.tp, CodeType::String));
            let line = &code_file.lines[6];
            assert_ne!(0, line.hash);
            assert_eq!(3, line.line.line_number);
            assert_eq!("value", line.value);
            assert_eq!(true, matches!(line.tp, CodeType::Code));
        }
    });

    // log::logger().flush();
}
