#[cfg(test)]
pub mod language_dictionary_tests {
    use log::info;
    use nvim_lang_core::{
        code::{
            code_file::CodeFile,
            programming::{ProgrammingLanguage, RUST},
        },
        common::logger::Logger,
        nvim_lang_dictionary::NvimLanguageDictionary,
        programming_lang::ProgrammingFile,
    };
    use rand::Rng;
    use std::{env, time::Duration};
    use tokio::{
        runtime::Runtime,
        task::JoinHandle,
        time::{self, Instant},
    };

    #[test]
    fn add_remove_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let mut nvim_language_dictionary = NvimLanguageDictionary::new(true);

        nvim_language_dictionary.append_word("lang".to_owned());
        nvim_language_dictionary.append_word("tokio".to_owned());
        nvim_language_dictionary.remove_word("tokio".to_owned());
        nvim_language_dictionary.append_word("tokio".to_owned());

        assert_eq!(2, nvim_language_dictionary.get_words().len());

        nvim_language_dictionary.append_word("nvim".to_owned());

        let words = nvim_language_dictionary.get_words();

        assert_eq!(3, words.len());
        assert_eq!("nvim".to_owned(), words[2]);

        nvim_language_dictionary.remove_word("tokio".to_owned());

        let words = nvim_language_dictionary.get_words();

        assert_eq!(2, words.len());
        assert_eq!("lang".to_owned(), words[0]);
        assert_eq!("nvim".to_owned(), words[1]);

        nvim_language_dictionary.remove_word("nvim".to_owned());
    }

    #[test]
    fn testing_threads() {
        env::set_var("RUST_BACKTRACE", "1");
        Logger::console_init();

        println!("Create tokio runtime");
        let tokio_runtime = Runtime::new().expect("Unable to start up Tokio Runtime {:#?}");

        let path = "/home/brent/Documents/pojects/nvim-lang-core/src/code/code_file.rs";
        let n = nvim_lang_core::programming_lang::ProgrammingLanguage::init();
        let k = Instant::now();
        let p = ProgrammingFile::create(path, &n[1]);

        let elapsed_time = k.elapsed(); // Stop the stopwatch
        println!("Total 2 execution time: {:.2?}", elapsed_time);
        // println!("{:#?}", p.lines);
        tokio_runtime.block_on(async {
            let k = Instant::now();
            let code_file = CodeFile::create(path, &RUST).await;

            let elapsed_time = k.elapsed();
            // println!("{:#?}", code_file.blocks);
            println!("{:#?}", code_file.lines);
            println!("Total execution time: {:.2?}", elapsed_time);
        });

        info!("This is the end");
        log::logger().flush();
    }
}
