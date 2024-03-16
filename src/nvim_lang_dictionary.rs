use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, IoSlice, Write},
};

use log::{error, info, warn};

#[derive(Debug)]
pub struct NvimLanguageDictionary<'nld> {
    file_name: &'nld str,
    words: Vec<String>,
}

impl<'nld> NvimLanguageDictionary<'nld> {
    pub fn new() -> Self {
        return Self {
            file_name: "nvim_language_dictionary.txt",
            words: get_words_and_create_open_dictionary("nvim_language_dictionary.txt"),
        };
    }

    pub fn append_word(&mut self, word: String) {
        // TODO: Check if the word exit before adding.
        let mut file = match File::open(self.file_name) {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open or create language dictionary {:#?}", e);
                return;
            }
        };

        if let Err(e) = writeln!(file, "{}", word) {
            error!(
                "Unable to append word {} language dictionary {:#?}",
                word, e
            );
            return;
        }

        self.words.push(word);
    }

    pub fn remove_word(&mut self, word: String) {
        let mut file = match File::open(self.file_name) {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open or create language dictionary {:#?}", e);
                return;
            }
        };
    }
}

fn get_words_and_create_open_dictionary(file_name: &str) -> Vec<String> {
    let file = OpenOptions::new().read(true).create(true).open(file_name);

    let file = match file {
        Ok(file) => file,
        Err(e) => {
            error!("Unable to open or create language dictionary {:#?}", e);
            return Vec::new();
        }
    };

    let file_buf_reader = BufReader::new(file);

    let lines = file_buf_reader.lines().map(|line| {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                error!("Unable to read a line in language dictionary {:#?}", e);
                return String::new();
            }
        };

        line
    });

    return lines.collect();
}
