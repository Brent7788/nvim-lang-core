use nvim_lang_core::lang_tool::NvimLangCoreData;

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

    fn data_len_to_be(len: usize, result: &NvimLangCoreData) {
        assert_eq!(false, result.is_empty());
        assert_eq!(len, result.data.len());
    }

    fn assert(&self, data_index: usize, result: &NvimLangCoreData) {
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

    use log::info;
    use nvim_lang_core::{common::logger::Logger, nvim_lang_core::NvimLangCore};

    use crate::Expected;

    #[tokio::test]
    async fn simple_comment_should_be() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        let file_path =
            "/home/brent/Documents/projects/nvim-lang-core/tests/file_test_cases/comments/simple_one_line_comment.rs";
        let core = NvimLangCore::new(None, None);

        let result = core.process_file(file_path.to_owned()).await;

        info!("{:#?}", result);

        log::logger().flush();

        Expected::data_len_to_be(1, &result);

        Expected::new(1, 10, 15, 6, "simle", vec!["simple", "smile", "simile"]).assert(0, &result);
    }
}
