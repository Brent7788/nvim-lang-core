use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use home::home_dir;
use log::{error, info};

#[derive(Debug)]
pub struct NvimLanguageReadonlyDictionary {
    words: Vec<String>,
}

#[derive(Debug)]
pub struct NvimLanguageDictionary {
    path: PathBuf,
    words: Vec<String>,
}

impl NvimLanguageDictionary {
    pub fn new(testing_only: bool) -> Self {
        let mut home_dir = match home_dir() {
            Some(home_dir) => home_dir,
            None => {
                error!("Unable to find home dirictory");
                return Self {
                    path: PathBuf::new(),
                    words: Vec::new(),
                };
            }
        };

        if testing_only {
            home_dir.push(".local/nvim_language_dictionary.txt");
        } else {
            home_dir.push(".local/share/nvim/nvim_language_dictionary.txt");
        }

        let words = get_words_and_create_open_dictionary(&home_dir);

        return Self {
            path: home_dir,
            words,
        };
    }

    pub fn to_readonly(&self) -> NvimLanguageReadonlyDictionary {
        return NvimLanguageReadonlyDictionary {
            words: self.words.clone(),
        };
    }

    pub fn get_words(&self) -> Vec<String> {
        return self.words.clone();
    }

    pub fn append_word(&mut self, mut word: String) {
        // INFO: Ignore word that already exit
        for (_, w) in self.words.iter().enumerate() {
            if w != &word {
                continue;
            }

            info!("Word '{}' already exit in your dictionary", word);
            return;
        }

        let mut file = match OpenOptions::new().append(true).open(&self.path) {
            Ok(file) => file,
            Err(e) => {
                error!("Unable to open or create language dictionary {:#?}", e);
                return;
            }
        };

        word = format!("\n{}", word);

        if let Err(e) = file.write(word.as_bytes()) {
            error!(
                "Unable to append word {} language dictionary {:#?}",
                word, e
            );
            return;
        }

        info!("Word '{}' added to your dictionary", word);
        self.words.push(word.replace("\n", ""));
    }

    pub fn remove_word(&mut self, word: String) {
        let mut file = match File::create(&self.path) {
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

        match file.write(self.words_to_string().as_bytes()) {
            Ok(_) => {
                info!("Word '{}' was removed from your dictionary", word);
            }
            Err(e) => {
                error!("Unable to write to language dictionary file {:#?}", e);
            }
        };
    }

    pub fn exit_in_dictionary(&self, value: &str) -> bool {
        for word in &self.words {
            if value == word {
                return true;
            }
        }

        return false;
    }

    pub fn replace_with_dictionary_values(&self, value: String) -> String {
        let values = value.split_whitespace();
        let mut new_value = String::with_capacity(value.len());

        'v: for value in values {
            for word in &self.words {
                if value == word {
                    continue 'v;
                }
            }

            new_value.push_str(value);
            new_value.push(' ');
        }

        return new_value;
    }

    fn words_to_string(&mut self) -> String {
        return self.words.join("\n");
    }
}

impl NvimLanguageReadonlyDictionary {
    pub fn exit_in_dictionary(&self, value: &str) -> bool {
        for word in &self.words {
            if value == word {
                return true;
            }
        }

        return false;
    }

    pub fn replace(&self, value: &str) -> String {
        let mut new_value = String::with_capacity(value.len());
        let value_split = value.split_whitespace();

        'ignore: for val in value_split {
            if val.is_empty() {
                continue;
            }

            for word in &self.words {
                if val == word {
                    continue 'ignore;
                }
            }
            new_value.push_str(val);
            new_value.push(' ');
        }

        if new_value.is_empty() {
            return value.to_owned();
        }

        return new_value;
    }
}

fn get_words_and_create_open_dictionary(path: &PathBuf) -> Vec<String> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path);

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
