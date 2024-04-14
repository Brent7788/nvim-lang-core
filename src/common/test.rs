use crate::nvim_lang::NvimLanguageFile;

const PROJECT_PATH: &str = "/home/brent/Documents/projects";

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

    pub fn data_len_to_be(len: usize, result: &NvimLanguageFile) {
        assert_eq!(false, result.is_empty());
        assert_eq!(len, result.nvim_lang_lines.len());
    }

    pub fn assert(&self, data_index: usize, result: &NvimLanguageFile) {
        let result = &result.nvim_lang_lines[data_index];

        assert_eq!(self.ln, result.line_number);
        assert_eq!(self.sc, result.start_column);
        assert_eq!(self.ec, result.end_column);
        assert_eq!(self.orig, result.options.original);
        assert_eq!(self.ol, result.options.options.len());
        for (index, option) in self.fopt.iter().enumerate() {
            assert_eq!(*option, result.options.options[index]);
        }
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
