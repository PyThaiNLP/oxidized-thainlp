use std::io::BufReader;
use std::{error::Error, io::prelude::*};
use std::{fs::File, path::PathBuf};

use super::super::fixed_bytes_str::four_bytes::CustomString;
use super::trie_char::TrieChar as Trie;

pub enum DictSource {
    FilePath(PathBuf),
    WordList(Vec<String>),
}

pub fn create_dict_trie(source: DictSource) -> Result<Trie, Box<dyn Error>> {
    match source {
        DictSource::FilePath(single_source) => {
            let file_reader = File::open(single_source.as_path());
            match file_reader {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    let mut line = String::with_capacity(50);
                    let mut dict: Vec<CustomString> = Vec::with_capacity(600);
                    while reader.read_line(&mut line).unwrap() != 0 {
                        dict.push(CustomString::new(&line));
                        line.clear();
                    }
                    dict.shrink_to_fit();
                    Ok(Trie::new(&dict))
                }
                Err(error) => Err(Box::from(error)),
            }
        }
        DictSource::WordList(word_list) => {
            let custom_word_list: Vec<CustomString> = word_list
                .into_iter()
                .map(|word| CustomString::new(&word))
                .collect();
            Ok(Trie::new(&custom_word_list))
        }
    }
}