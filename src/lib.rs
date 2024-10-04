use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct TwoColumnsTableData {
    penguin_data: String,
}

#[derive(Debug, Deserialize)]
struct SearchWordData {
    search_word: String,
    search_text: String,
}

#[derive(Debug, Deserialize)]
struct CommaSeparatedNumericData {
    integer_data_string: String,
}

#[derive(Debug, Deserialize)]
struct PigLatin {
    text_for_pig_latin_conversion: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    two_columns_table_data: TwoColumnsTableData,
    search_word_data: SearchWordData,
    comma_separated_numeric_data: CommaSeparatedNumericData,
    pig_latin: PigLatin,
}

fn load_config_file() -> Config {
    let config_content = fs::read_to_string("config.toml").unwrap();

    // Parse the string into the Config struct
    toml::from_str(&config_content).unwrap()
}

pub fn get_penguin_data_string() -> String {
    let config: Config = load_config_file();
    config.two_columns_table_data.penguin_data
}

pub fn get_search_word_data() -> (String, String) {
    let config = load_config_file();
    (
        config.search_word_data.search_word,
        config.search_word_data.search_text,
    )
}
pub fn get_raw_integer_data() -> Vec<i32> {
    let config: Config = load_config_file();
    let raw_integer_data_string = config.comma_separated_numeric_data.integer_data_string;
    let mut integers_vector = Vec::new();
    for one_integer_string in raw_integer_data_string.split(',') {
        let one_integer = one_integer_string
            .trim()
            .parse()
            .expect("Not a valid number");
        integers_vector.push(one_integer);
    }

    integers_vector
}

pub fn get_text_for_pig_latin_conversion() -> String {
    let config: Config = load_config_file();
    config.pig_latin.text_for_pig_latin_conversion
}

pub fn is_this_a_vowel_word(word: &str) -> bool {
    match word.chars().next() {
        Some(c) => "AEIOUaeiou".contains(c),
        None => false,
    }
}

pub fn is_this_a_vowel_char(one_char: char) -> bool {
    let mut is_vowel: bool = false;
    if "AEIOUaeiou".contains(one_char) {
        is_vowel = true;
    }
    is_vowel
}
