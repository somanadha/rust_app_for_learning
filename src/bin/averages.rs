#![allow(warnings)]
use rust_app_for_learning::get_raw_integer_data;
use std::collections::HashMap;

fn main() {
    let mut integers_vector = get_raw_integer_data();
    println!("Original numbers: {integers_vector:?}");

    integers_vector.sort();
    println!("Sorted numbers: {integers_vector:?}");

    let average = get_mean();
    println!("Average of above numbers is {average}");

    let median = get_median();
    println!("Median of above numbers is {median}");

    let mode = get_mode();
    println!("Mode of above numbers is {mode}");
}

pub fn get_mean() -> f32 {
    let integer_vector = get_raw_integer_data();
    let sum: i32 = integer_vector.iter().sum();

    sum as f32 / integer_vector.len() as f32
}

pub fn get_median() -> f32 {
    let mut sorted_integer_vector = get_raw_integer_data();
    sorted_integer_vector.sort();
    let integer_vector_length = sorted_integer_vector.len();
    let median = if integer_vector_length % 2 != 0 {
        let middle_term_index = integer_vector_length / 2;
        *sorted_integer_vector.get(middle_term_index).unwrap() as f32
    } else {
        let middle_term_index_1 = integer_vector_length / 2;
        let middle_term_index_2 = integer_vector_length / 2 - 1;

        let middle_term_1 = *sorted_integer_vector.get(middle_term_index_1).unwrap() as f32;
        let middle_term_2 = *sorted_integer_vector.get(middle_term_index_2).unwrap() as f32;

        (middle_term_1 + middle_term_2) / 2_f32
    };

    median
}

pub fn get_mode() -> f32 {
    let integer_vector = get_raw_integer_data();
    let mut integer_repeat_count_hash_map = HashMap::new();
    let mut max_repeat_count = 0;
    let mut mode_value = f32::NAN;
    for one_integer in integer_vector {
        let count = integer_repeat_count_hash_map
            .entry(one_integer)
            .or_insert(0);
        *count += 1;

        if *count > max_repeat_count {
            max_repeat_count = *count;
            mode_value = one_integer as f32;
        }
    }

    mode_value
}
