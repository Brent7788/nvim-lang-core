pub mod code_file_code_tests;
pub mod code_file_comment_tests;
pub mod code_file_string_tests;

use log::info;
use rstest::rstest;
use std::env;
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
// #[case("/lua/strings/simple_string.lua", ProgrammingLanguageType::Lua)]
fn edge_case_should_be(#[case] path: &str, #[case] lang_type: ProgrammingLanguageType) {
    // env::set_var("RUST_BACKTRACE", "1");
    Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        match lang_type {
            ProgrammingLanguageType::Lua => {
                assert_code_file(CodeFile::create(&file_path, &LUA).await)
            }
            ProgrammingLanguageType::Rust => {
                assert_code_file(CodeFile::create(&file_path, &RUST).await)
            }
        }

        fn assert_code_file<const OPERATOR_COUNT: usize, const RESERVED_KEYWORD_COUNT: usize>(
            code_file: CodeFile<OPERATOR_COUNT, RESERVED_KEYWORD_COUNT>,
        ) {
            assert_eq!(6, code_file.lines.len());
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
            assert_eq!(true, matches!(line.tp, CodeType::Code));
        }
    });

    log::logger().flush();
}
