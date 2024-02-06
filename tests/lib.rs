use nvim_lang_core::nvim_lang::NvimLanguageFile;

#[derive(Debug)]
struct Expected<'r> {
    ln: usize,
    sc: usize,
    ec: usize,
    ol: usize,
    orig: &'r str,
    fopt: Vec<&'r str>,
}

impl<'r> Expected<'r> {
    fn new(ln: usize, sc: usize, ec: usize, ol: usize, orig: &'r str, fopt: Vec<&'r str>) -> Self {
        return Self {
            ln,
            sc,
            ec,
            ol,
            orig,
            fopt,
        };
    }

    fn data_len_to_be(len: usize, result: &NvimLanguageFile) {
        assert_eq!(false, result.is_empty());
        assert_eq!(len, result.data.len());
    }

    fn assert(&self, data_index: usize, result: &NvimLanguageFile) {
        let result = &result.data[data_index];

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

#[cfg(test)]
pub mod tests {
    use std::env;

    use nvim_lang_core::{common::logger::Logger, nvim_lang_core::NvimLangCore};

    use crate::Expected;

    const PROJECT_PATH: &str = "/home/brent/Documents/projects";

    const TEST_FILE_PATH: &str = "/nvim-lang-core/tests/file_test_cases";
    const TEST_COMMENT_PATH: &str = "/comments";

    #[tokio::test]
    async fn simple_comment_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_comment_path("/simple_one_line_comment.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path).await;

        log::logger().flush();

        Expected::data_len_to_be(1, &result);
        Expected::new(1, 10, 15, 6, "simle", vec!["simple", "smile", "simile"]).assert(0, &result);
    }

    #[tokio::test]
    async fn multiple_comment_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_comment_path("/multiple_comments.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path).await;

        log::logger().flush();

        Expected::data_len_to_be(6, &result);
        Expected::new(1, 16, 26, 1, "commmented", vec!["commented"]).assert(0, &result);
        Expected::new(2, 21, 29, 2, "invoving", vec!["involving", "invoking"]).assert(1, &result);
        Expected::new(4, 2, 3, 1, "a", vec!["A"]).assert(2, &result);
        Expected::new(4, 14, 21, 5, "brances", vec!["branches"]).assert(3, &result);
        Expected::new(4, 38, 47, 2, "especialy", vec!["especially"]).assert(4, &result);
        Expected::new(4, 79, 85, 2, "prduct", vec!["product"]).assert(5, &result);
    }

    #[tokio::test]
    async fn comment_block_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_comment_path("/comment_block.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path).await;

        log::logger().flush();

        Expected::data_len_to_be(6, &result);
        Expected::new(1, 17, 27, 1, "commmented", vec!["commented"]).assert(0, &result);
        Expected::new(2, 19, 27, 2, "invoving", vec!["involving", "invoking"]).assert(1, &result);
        Expected::new(4, 0, 1, 1, "a", vec!["A"]).assert(2, &result);
        Expected::new(4, 12, 19, 5, "brances", vec!["branches"]).assert(3, &result);
        Expected::new(4, 36, 45, 2, "especialy", vec!["especially"]).assert(4, &result);
        Expected::new(4, 77, 83, 2, "prduct", vec!["product"]).assert(5, &result);
    }

    #[tokio::test]
    async fn full_comment_should_be() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        let file_path = get_test_comment_path("/full_comment.rs");

        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path).await;

        // info!("{:#?}", result);

        log::logger().flush();

        Expected::data_len_to_be(15, &result);
        Expected::new(3, 34, 42, 1, "too have", vec!["to have"]).assert(0, &result);
        Expected::new(4, 43, 50, 1, "colours", vec!["colors"]).assert(1, &result);
        Expected::new(4, 65, 73, 2, "seplling", vec!["selling", "spelling"]).assert(2, &result);
        Expected::new(4, 90, 100, 1, "underilnes", vec!["underlines"]).assert(3, &result);
        Expected::new(5, 4, 15, 1, "Furthermore", vec!["Furthermore,"]).assert(4, &result);
        Expected::new(5, 24, 31, 1, "error's", vec!["errors"]).assert(5, &result);
        Expected::new(6, 41, 61, 1, "in a reliable manner", vec!["reliably"]).assert(6, &result);
        Expected::new(7, 4, 7, 1, "did", vec!["Did"]).assert(7, &result);
        Expected::new(7, 46, 61, 1, "double clicking", vec!["double-clicking"]).assert(8, &result);
        Expected::new(7, 70, 73, 1, "Its", vec!["It's"]).assert(9, &result);
        Expected::new(7, 74, 75, 1, "a", vec!["an"]).assert(10, &result);
        Expected::new(8, 33, 37, 1, "youd", vec!["you'd"]).assert(11, &result);
        Expected::new(8, 68, 78, 1, "over sea's", vec!["overseas"]).assert(12, &result);
        Expected::new(9, 18, 37, 1, "PM in the afternoon", vec!["PM"]).assert(13, &result);
        Expected::new(9, 41, 60, 0, "Monday, 27 May 2007", vec![]).assert(14, &result);
    }

    fn get_test_comment_path(test_file: &str) -> String {
        return String::new() + PROJECT_PATH + TEST_FILE_PATH + TEST_COMMENT_PATH + test_file;
    }
}
