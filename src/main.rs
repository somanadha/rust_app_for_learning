#![allow(warnings)]
use std::ops::Add;
use std::process::Output;
use std::time::{Duration, Instant};
use std::usize;
use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
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

    // let mut x: String = String::from("Hello");
    // let mut y = &mut x;
    // let z = &mut y;
    // z.push_str(" Rusty");
    // y.push_str(" World");
    // x.push_str("!");
    //println!("{x}");
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
