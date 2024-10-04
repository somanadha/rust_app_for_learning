use rust_app_for_learning::get_text_for_pig_latin_conversion;
use rust_app_for_learning::is_this_a_vowel_char;
use rust_app_for_learning::is_this_a_vowel_word;
fn main() {
    print_normal_text_in_pig_latin();

    //     let x = convert_to_pig_latin("The");
    //     println!("{x}");
}

pub fn print_normal_text_in_pig_latin() {
    let text_for_pig_latin_conversion = get_text_for_pig_latin_conversion();

    println!("---------------------------------------------");
    println!("               Original Text");
    println!("---------------------------------------------");
    println!("{text_for_pig_latin_conversion}");
    println!("---------------------------------------------");
    println!();

    let mut pig_latin_converted_text = String::new();
    for one_line in text_for_pig_latin_conversion.lines() {
        let words_in_one_line = one_line.split("\t ");
        let mut converted_line = String::new();
        for word in words_in_one_line {
            let converted_word = convert_to_pig_latin(word);
            converted_line.insert_str(converted_word.len(), &converted_word);
            converted_line.insert(' '.len_utf8(), ' ');
        }
        let trimmed_converted_line = converted_line.trim();
        pig_latin_converted_text.insert_str(trimmed_converted_line.len(), trimmed_converted_line);
        pig_latin_converted_text.insert_str("\n".len(), "\n");
    }
    println!("---------------------------------------------");
    println!("               Conveted Text");
    println!("---------------------------------------------");
    println!("{pig_latin_converted_text}");
    println!("---------------------------------------------");
}

pub fn convert_to_pig_latin(word: &str) -> String {
    let mut converted_string: String = String::new();
    if is_this_a_vowel_word(word) {
        converted_string.insert_str(0, word);
        converted_string.insert_str(word.len(), "hay");
    } else {
        let mut word_chars_iterator = word.chars();

        let mut initial_consonants: Vec<char> = Vec::new();

        for one_char in word_chars_iterator.by_ref() {
            println!("{:?}", one_char);
            if is_this_a_vowel_char(one_char) {
                converted_string.insert(converted_string.len(), one_char);
                break;
            }
            initial_consonants.push(one_char);
        }

        println!("{:?}", initial_consonants);

        for one_char in word_chars_iterator.by_ref() {
            converted_string.insert(converted_string.len(), one_char);
        }

        for one_char in initial_consonants {
            converted_string.insert(converted_string.len(), one_char);
        }

        converted_string.insert_str(converted_string.len(), "ay");
    }
    converted_string
}
