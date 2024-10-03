use serde::Deserialize;
use std::{collections::HashMap, fs};

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
struct RawData {
    raw_integer_data_string: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    two_columns_table_data: TwoColumnsTableData,
    search_word_data: SearchWordData,
    raw_data: RawData,
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

pub fn print_table_data(table_data: &str) {
    println!("Table Data");
    println!("===========");

    let table_data_as_lines = table_data.lines();

    for (line_index, one_line) in table_data_as_lines.enumerate() {
        if line_index == 0 || one_line.trim().is_empty() {
            continue;
        }

        let tokens_in_one_line: Vec<_> = one_line
            .split(',')
            .map(|one_token| one_token.trim())
            .collect();

        if tokens_in_one_line.len() >= 2 {
            let length_value_result: Result<f32, _> = tokens_in_one_line[1].parse();

            if length_value_result.is_ok() {
                println!(
                    "{:?} - {:?}",
                    tokens_in_one_line[0].trim(),
                    length_value_result.unwrap()
                )
            }
        }
    }
}

pub fn get_search_word_data() -> (String, String) {
    let config = load_config_file();
    (
        config.search_word_data.search_word,
        config.search_word_data.search_text,
    )
}

pub fn get_search_word_lines() -> HashMap<usize, String> {
    let mut search_word_lines = HashMap::new();
    let search_word_data = get_search_word_data();
    let search_word = search_word_data.0;
    let search_text = search_word_data.1;
    for (i, line) in search_text.lines().enumerate() {
        if line.contains(&search_word) {
            search_word_lines.insert(i + 1, line.to_string());
        }
    }
    search_word_lines
}

pub fn print_search_word_lines() {
    let search_word_data = get_search_word_data();
    println!();
    println!(
        "The searh word: \"{}\" found in following lines (prefixed with line numbers)",
        search_word_data.0
    );
    println!("=============================================================================");
    for (line_num, line) in get_search_word_lines() {
        print!("{} : {}", line_num, line)
    }
}

pub fn get_raw_integer_data() -> Vec<i32> {
    let config: Config = load_config_file();
    let raw_integer_data_string = config.raw_data.raw_integer_data_string;
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
