use log::info;
use rstest::rstest;
use std::env;
use tokio::runtime::{self, Runtime};

use nvim_lang_core::{
    code::{
        code_file::{CodeFile, CodeType},
        programming::{LUA, RUST},
    },
    common::{logger::Logger, test::get_project_path},
};

#[rstest]
#[case(
    "/rust/comments/simple_one_line_comment.rs",
vec![(1,1,"This is simle one line comment test case.","//This is simle one line comment test case.")])]
#[case(
    "/rust/comments/multiple_comments.rs",
vec![
        (4,1,"This is multi commmented line.","//This is multi commmented line."),
        (4,2,"Multiple having or invoving several parts, elements, or members.","//Multiple having or invoving several parts, elements, or members."),
        (4,3,"","//"),
        (4,4,"a shop with brances in many places, especialy one selling a specific type of prduct.","//a shop with brances in many places, especialy one selling a specific type of prduct."),
    ])]
fn rust_comment_should_be(#[case] path: &str, #[case] values: Vec<(usize, usize, &str, &str)>) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let code_file = CodeFile::create(&file_path, &RUST).await;
        for (index, data) in values.iter().enumerate() {
            assert_eq!(data.0, code_file.lines.len());
            let line = &code_file.lines[index];
            assert_ne!(0, line.hash);
            assert_eq!(data.1, line.line.line_number);
            assert_eq!(data.2, line.value);
            assert_eq!(data.3, line.line.original_line);
            assert_eq!(true, matches!(line.tp, CodeType::Comment));
        }
    });

    // log::logger().flush();
}

#[rstest]
#[case(
    "/rust/comments/comment_block.rs",
vec![(1,4, r#"This is multi commmented line.
Multiple having or invoving several parts, elements, or members.

a shop with brances in many places, especialy one selling a specific type of prduct."#)])]
#[case(
    "/rust/comments/full_comment.rs",
vec![(1,10, r#"LanguageTool is your intelligent writing assistant for all common browsers and word processors.
    Write or paste your text here too have it checked continuously.
    Errors will be underlined in different colours: we will mark seplling errors with red underilnes.
    Furthermore grammar error's are highlighted in yellow.
    LanguageTool also marks style issues in a reliable manner by underlining them in blue.
    did you know that you can sea synonyms by double clicking a word? Its a impressively
    versatile tool especially if youd like to tell a colleague from over sea's about what
    happened at 5 PM in the afternoon on Monday, 27 May 2007."#)])]

fn rust_block_comment_should_be(#[case] path: &str, #[case] values: Vec<(usize, usize, &str)>) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let code_file = CodeFile::create(&file_path, &RUST).await;
        for (index, data) in values.iter().enumerate() {
            assert_eq!(data.0, code_file.blocks.len());
            let block = &code_file.blocks[index];
            assert_ne!(0, block.hash);
            assert_eq!(data.1, block.lines.len());
            assert_eq!(data.2, block.block);
        }
    });

    // log::logger().flush();
}

#[rstest]
#[case(
    "/lua/comments/simple_one_line_comment.lua",
vec![(1,1,"This is simle one line comment test case.","--This is simle one line comment test case.")])]
#[case(
    "/lua/comments/multiple_comments.lua",
vec![
        (4,1,"This is multi commmented line.","--This is multi commmented line."),
        (4,2,"Multiple having or invoving several parts, elements, or members.","--Multiple having or invoving several parts, elements, or members."),
        (4,3,"","--"),
        (4,4,"a shop with brances in many places, especialy one selling a specific type of prduct.","--a shop with brances in many places, especialy one selling a specific type of prduct."),
    ])]
fn lua_comment_should_be(#[case] path: &str, #[case] values: Vec<(usize, usize, &str, &str)>) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let code_file = CodeFile::create(&file_path, &LUA).await;
        for (index, data) in values.iter().enumerate() {
            assert_eq!(data.0, code_file.lines.len());
            let line = &code_file.lines[index];
            assert_ne!(0, line.hash);
            assert_eq!(data.1, line.line.line_number);
            assert_eq!(data.2, line.value);
            assert_eq!(data.3, line.line.original_line);
            assert_eq!(true, matches!(line.tp, CodeType::Comment));
        }
    });

    // log::logger().flush();
}

#[rstest]
#[case(
    "/lua/comments/comment_block.lua",
vec![(1,4, r#"This is multi commmented line.
Multiple having or invoving several parts, elements, or members.

a shop with brances in many places, especialy one selling a specific type of prduct."#)])]
#[case(
    "/lua/comments/full_comment.lua",
vec![(1,10, r#"LanguageTool is your intelligent writing assistant for all common browsers and word processors.
    Write or paste your text here too have it checked continuously.
    Errors will be underlined in different colours: we will mark seplling errors with red underilnes.
    Furthermore grammar error's are highlighted in yellow.
    LanguageTool also marks style issues in a reliable manner by underlining them in blue.
    did you know that you can sea synonyms by double clicking a word? Its a impressively
    versatile tool especially if youd like to tell a colleague from over sea's about what
    happened at 5 PM in the afternoon on Monday, 27 May 2007."#)])]

fn lua_block_comment_should_be(#[case] path: &str, #[case] values: Vec<(usize, usize, &str)>) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let code_file = CodeFile::create(&file_path, &LUA).await;
        // info!("{:#?}", code_file.blocks);
        for (index, data) in values.iter().enumerate() {
            assert_eq!(data.0, code_file.blocks.len());
            let block = &code_file.blocks[index];
            assert_ne!(0, block.hash);
            assert_eq!(data.1, block.lines.len());
            assert_eq!(data.2, block.block);
        }
    });

    // log::logger().flush();
}
