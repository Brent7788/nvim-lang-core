#[cfg(test)]
pub mod first_test {
    use std::env;

    use nvim_lang_core::{common::logger::Logger, nvim_lang_core::NvimLangCore};

    #[tokio::test]
    async fn sec() {
        Logger::console_init();
        env::set_var("RUST_BACKTRACE", "1");

        let core = NvimLangCore::new(None, None);

        core.process_file(
            "/home/brent/Documents/projects/nvim-lang-core/tests/file_test_cases/person.rs"
                .to_owned(),
        )
        .await;

        log::logger().flush();
    }
}
