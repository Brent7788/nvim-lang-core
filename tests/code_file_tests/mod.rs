pub mod code_file_code_tests;
pub mod code_file_comment_tests;
pub mod code_file_string_tests;

use log::info;
use nvim_lang_core::nvim_lang_dictionary::NvimLanguageDictionary;
use rstest::rstest;
use std::env;
use std::sync::Arc;
use tokio::runtime::{self, Runtime};

use nvim_lang_core::{
    code::{
        code_file::{CodeFile, CodeType},
        programming::{ProgrammingLanguage, ProgrammingLanguageType, LUA, RUST},
    },
    common::{logger::Logger, test::get_project_path},
};

#[rstest]
#[case("/rust/edge_case.rs", ProgrammingLanguageType::Rust)]
#[case("/lua/edge_case.lua", ProgrammingLanguageType::Lua)]
fn edge_case_should_be(#[case] path: &str, #[case] lang_type: ProgrammingLanguageType) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let nvim_language_dictionary = NvimLanguageDictionary::new(true);
        let code_file = CodeFile::new(file_path, Arc::new(nvim_language_dictionary.to_readonly()))
            .await
            .unwrap();

        assert_eq!(10, code_file.lines.len());
        let line = &code_file.lines[0];
        assert_ne!(0, line.hash);
        assert_eq!(2, line.line.line_number);
        assert_eq!("percon", line.value);
        assert_eq!(true, matches!(line.tp, CodeType::String));
        let line = &code_file.lines[1];
        assert_ne!(0, line.hash);
        assert_eq!(2, line.line.line_number);
        assert_eq!("value API Value", line.value);
        assert_eq!(true, matches!(line.tp, CodeType::Code));
        let line = &code_file.lines[2];
        assert_ne!(0, line.hash);
        assert_eq!(4, line.line.line_number);
        assert_eq!("percon", line.value);
        assert_eq!(true, matches!(line.tp, CodeType::String));
        let line = &code_file.lines[3];
        assert_ne!(0, line.hash);
        assert_eq!(4, line.line.line_number);
        assert_eq!("This is \"a Cliant", line.value);
        assert_eq!(true, matches!(line.tp, CodeType::String));
        let line = &code_file.lines[4];
        assert_ne!(0, line.hash);
        assert_eq!(4, line.line.line_number);
        assert_eq!("next", line.value);
        assert_eq!(true, matches!(line.tp, CodeType::String));
        let line = &code_file.lines[5];
        assert_ne!(0, line.hash);
        assert_eq!(4, line.line.line_number);
        assert_eq!("NEO Vim API", line.value);
    });

    // log::logger().flush();
}
//
#[rstest]
#[case("/rust/edge_case.rs", ProgrammingLanguageType::Rust)]
#[case("/lua/edge_case.lua", ProgrammingLanguageType::Lua)]
fn edge_case_2_should_be(#[case] path: &str, #[case] lang_type: ProgrammingLanguageType) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let nvim_language_dictionary = NvimLanguageDictionary::new(true);
        let code_file = CodeFile::new(file_path, Arc::new(nvim_language_dictionary.to_readonly()))
            .await
            .unwrap();

        assert_eq!(0, code_file.blocks.len());
        let line = &code_file.lines[6];
        assert_ne!(0, line.hash);
        assert_eq!(5, line.line.line_number);
        assert_eq!("Start '\" block", line.value);
        assert_eq!(true, matches!(line.tp, CodeType::Comment));
        let line = &code_file.lines[7];
        assert_ne!(0, line.hash);
        assert_eq!(5, line.line.line_number);
        assert_eq!(r##"This is "# string block"##, line.value);
        assert_eq!(true, matches!(line.tp, CodeType::String));
        let line = &code_file.lines[8];
        assert_ne!(0, line.hash);
        assert_eq!(5, line.line.line_number);
        assert_eq!(r#"This is a "block""#, line.value);
        assert_eq!(true, matches!(line.tp, CodeType::Comment));
        let line = &code_file.lines[9];
        assert_ne!(0, line.hash);
        assert_eq!(5, line.line.line_number);
        assert_eq!(r#"This is "end comment"#, line.value);
        assert_eq!(true, matches!(line.tp, CodeType::Comment));
    });

    // log::logger().flush();
}
