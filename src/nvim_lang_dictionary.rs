use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use log::{error, info};

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

    pub fn get_words(&self) -> Vec<String> {
        return self.words.clone();
    }

    pub fn append_word(&mut self, word: String) {
        // TODO: Check if the word exit before adding.
        let mut file = match OpenOptions::new().append(true).open(self.file_name) {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open or create language dictionary {:#?}", e);
                return;
            }
        };

        if let Err(e) = file.write(word.as_bytes()) {
            error!(
                "Unable to append word {} language dictionary {:#?}",
                word, e
            );
            return;
        }

        self.words.push(word);
    }

    pub fn remove_word(&mut self, word: String) {
        let mut file = match File::create(self.file_name) {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open language dictionary {:#?}", e);
                return;
            }
        };

        let mut was_removed = false;

        for (i, w) in self.words.iter().enumerate() {
            if w != &word {
                continue;
            }

            self.words.remove(i);
            was_removed = true;
            break;
        }

        if !was_removed {
            error!(
                "Unable to remove word '{}' from your language dictionary",
                word
            );
        }

        info!("HELLO: {:?}", self.words_to_string());

        match file.write(self.words_to_string().as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                error!("Unable to write to language dictionary file {:#?}", e);
            }
        };
    }

    fn words_to_string(&mut self) -> String {
        return self.words.join("\n");
    }
}

fn get_words_and_create_open_dictionary(file_name: &str) -> Vec<String> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name);

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
