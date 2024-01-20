use std::fs::OpenOptions;

use ::fast_log::Config;
use fast_log::fast_log;

#[derive(Debug)]
pub struct Logger;

impl Logger {
    pub fn console_init() {
        fast_log::init(Config::new().console().chan_len(Some(10000)))
            .expect("Console log did not init!");
    }

    //TODO: Need to create the nvim-lang.log file if not exit!
    pub fn file_init(mut chan_len: Option<usize>) {
        //TODO: Should use File::create
        let _ = OpenOptions::new().create_new(true).open("nvim-lang.log");

        if let None = chan_len {
            chan_len = Some(1000)
        }

        fast_log::init(Config::new().file("nvim-lang.log").chan_len(chan_len))
            .expect("File log did not init!");
    }
}
