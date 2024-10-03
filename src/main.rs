#![allow(warnings)]
use rust_app_for_learning::get_penguin_data_string;
use rust_app_for_learning::print_search_word_lines;
use rust_app_for_learning::print_table_data;
use std::ops::Add;
use std::process::Output;
use std::time::{Duration, Instant};
fn main() {
    //print_table_data(&get_penguin_data_string());

    // let a = 10;
    // let b = 20;

    // println!("Adding 32 bit integers ({a}, {b}) using generic method: {:?}", generic_add(a, b));

    // let a = 10.0;
    // let b = 20.0;

    // println!("Adding 64bit floating number ({a}, {b}) using generic method: {:?}", generic_add(a, b));

    // let a = Duration::new(10, 0);
    // let b = Duration::new(20,0);

    // println!("Adding Duration objects ({:?}, {:?}) using generic method: {:?}", a, b, generic_add(a, b));
    //print_search_word_lines();
}

fn add_with_lifetimes<'d, 'e>(i: &'d i32, j: &'e i32) -> i32 {
    *i + *j
}

fn add_with_out_lifetimes(i: &i32, j: &i32) -> i32 {
    *i + *j
}

fn generic_add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
