#[cfg(test)]
pub mod common_tests {
    use std::env;

    use log::info;
    use nvim_lang_core::common::{
        logger::Logger,
        string::{DelimiterType, StringDelimiterSlice},
    };

    #[test]
    fn simple_string_slices_by_should_be() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        let n = String::from("var x = 'This should be simple.';");

        info!("{:?}", n);
        let n_slices: [Option<&str>; 2] =
            n.slices_by(&DelimiterType::DelimiterChar('\''), &[DelimiterType::None]);

        info!("{:?}", n_slices);

        log::logger().flush();

        assert_ne!(None, n_slices[0]);

        if let Some(s) = n_slices[0] {
            assert_eq!("'This should be simple.'", s);
        }
    }
}
