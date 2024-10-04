use rust_app_for_learning::get_search_word_data;
use std::collections::HashMap;
fn main() {
    print_search_word_lines();
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
