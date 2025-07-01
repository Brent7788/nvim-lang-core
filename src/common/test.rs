use crate::nvim_lang::NvimLanguageFile;

const PROJECT_PATH: &str = "/home/brent/Documents/pojects";

const TEST_FILE_PATH: &str = "/nvim-lang-core/tests/file_test_cases";
const TEST_COMMENT_PATH: &str = "/comments";
const TEST_CODE_PATH: &str = "/codes";
const TEST_STRING_PATH: &str = "/strings";

const BENCH_PATH: &str = "/nvim-lang-core/src/programming_lang.rs";

#[derive(Debug)]
pub struct Expected<'r> {
    pub ln: usize,
    pub sc: usize,
    pub ec: usize,
    pub ol: usize,
    pub orig: &'r str,
    pub fopt: Vec<&'r str>,
}

impl<'r> Expected<'r> {
    pub fn new(
        ln: usize,
        sc: usize,
        ec: usize,
        ol: usize,
        orig: &'r str,
        fopt: Vec<&'r str>,
    ) -> Self {
        return Self {
            ln,
            sc,
            ec,
            ol,
            orig,
            fopt,
        };
    }

    pub fn data_len_to_be(len: usize, result: &crate::nvim_language::file::NvimLanguageFile) {
        assert_eq!(len, result.nvim_lang_lines.len());
    }

    pub fn assert(&self, data_index: usize, result: &crate::nvim_language::file::NvimLanguageFile) {
        let result = &result.nvim_lang_lines[data_index];

        assert_eq!(self.ln, result.line_number, "line_number");
        assert_eq!(self.sc, result.start_column, "start_column");
        assert_eq!(self.ec, result.end_column, "end_column");
        assert_eq!(self.orig, result.options.original, "original");
        assert!(
            result.options.options.len() >= self.fopt.len(),
            "NvimLanguageLine.options length must be >= then the test options"
        );

        for option in &self.fopt {
            let no_match = "No Match Find in NvimLanguageLine.options".to_owned();
            let r_match = result
                .options
                .options
                .iter()
                .find(|r_option| r_option == option)
                .unwrap_or(&no_match);

            assert_eq!(option, r_match);
        }
    }
}

pub trait ExpectedTrait {
    fn expected_sorting_order(&mut self);
}

impl ExpectedTrait for Vec<Expected<'_>> {
    fn expected_sorting_order(&mut self) {
        self.sort_by(|e, b| e.ln.cmp(&b.ln).then_with(|| e.sc.cmp(&b.sc)));
    }
}

pub fn get_project_path(path: &str) -> String {
    return String::new() + PROJECT_PATH + TEST_FILE_PATH + path;
}

pub fn get_test_comment_path(test_file: &str) -> String {
    return String::new() + PROJECT_PATH + TEST_FILE_PATH + TEST_COMMENT_PATH + test_file;
}

pub fn get_test_code_path(test_file: &str) -> String {
    return String::new() + PROJECT_PATH + TEST_FILE_PATH + TEST_CODE_PATH + test_file;
}

pub fn get_test_code_string_path(test_file: &str) -> String {
    return String::new() + PROJECT_PATH + TEST_FILE_PATH + TEST_STRING_PATH + test_file;
}

pub fn get_bench_path() -> String {
    return String::new() + PROJECT_PATH + BENCH_PATH;
}
