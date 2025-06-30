use log::info;
use rstest::rstest;
use std::env;
use std::sync::Arc;
use tokio::runtime::{self, Runtime};

use nvim_lang_core::{
    code::{
        code_file::{CodeFile, CodeType},
        programming::{LUA, RUST},
    },
    common::{logger::Logger, test::get_project_path},
    nvim_lang_dictionary::NvimLanguageDictionary,
};

#[rstest]
#[case(
    "/rust/codes/simple_code.rs",
vec![(1,1,"upercase prduct String","pub fn upercase(prduct: String) {}")])]
#[case(
    "/rust/codes/multiple_code.rs",
vec![
        (8,1,"derive Debug","#[derive(Debug)]"),
        (8,2,"Main Foldr","pub struct MainFoldr {"),
        (8,3,"foldr path String","    foldr_path: String,"),
        (8,6,"Main Foldr","impl MainFoldr {"),
        (8,7,"generte foldr systim String procces Main Foldr","    pub fn generte_foldr(systim: String, procces: u32) -> MainFoldr {"),
        (8,8,"procces","        if procces == 0 {"),
        (8,9,"panic","            panic!(\"\");"),
        (8,12,"Main Foldr foldr path systim","        return MainFoldr { foldr_path: systim };"),
    ])]
fn rust_code_should_be(#[case] path: &str, #[case] values: Vec<(usize, usize, &str, &str)>) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let nvim_language_dictionary = NvimLanguageDictionary::new(true);
        let code_file = CodeFile::new(file_path, Arc::new(nvim_language_dictionary.to_readonly()))
            .await
            .unwrap();

        // info!("{:#?}", code_file.lines);
        for (index, data) in values.iter().enumerate() {
            assert_eq!(data.0, code_file.lines.len());
            let line = &code_file.lines[index];
            assert_ne!(0, line.hash);
            assert_eq!(data.1, line.line.line_number);
            assert_eq!(data.2, line.value);
            assert_eq!(data.3, line.line.original_line);
            assert_eq!(true, matches!(line.tp, CodeType::Code));
        }
    });

    log::logger().flush();
}

#[rstest]
#[case(
    "/lua/codes/simple_code.lua",
vec![(1,2,"upercase prduct","    upercase = function(prduct)", false)])]
#[case(
    "/lua/codes/multiple_code.lua",
vec![
        (7,1,"Main Foldr","local MainFoldr = {", false),
        (7,2,"defaullt","    foldr_path = 'defaullt'", true),
        (7,2,"foldr path","    foldr_path = 'defaullt'", false),
        (7,5,"Main Foldr generte foldr systim procces","function MainFoldr.generte_foldr(systim, procces)", false),
        (7,6,"procces","    if procces == 0 then", false),
        (7,11,"foldr path systim","        foldr_path = systim", false),
        (7,15,"Main Foldr","return MainFoldr", false),
    ])]
fn lua_code_should_be(#[case] path: &str, #[case] values: Vec<(usize, usize, &str, &str, bool)>) {
    // env::set_var("RUST_BACKTRACE", "1");
    // Logger::console_init();
    let runtime = Runtime::new().expect("");

    let file_path = get_project_path(path);

    runtime.block_on(async {
        let nvim_language_dictionary = NvimLanguageDictionary::new(true);
        let code_file = CodeFile::new(file_path, Arc::new(nvim_language_dictionary.to_readonly()))
            .await
            .unwrap();

        for (index, data) in values.iter().enumerate() {
            assert_eq!(data.0, code_file.lines.len());
            let line = &code_file.lines[index];
            assert_ne!(0, line.hash);
            assert_eq!(data.1, line.line.line_number);
            assert_eq!(data.2, line.value);
            assert_eq!(data.3, line.line.original_line);

            if data.4 {
                assert_eq!(true, matches!(line.tp, CodeType::String));
            } else {
                assert_eq!(true, matches!(line.tp, CodeType::Code));
            }
        }
    });

    // log::logger().flush();
}
