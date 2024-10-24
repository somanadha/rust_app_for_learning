#![allow(warnings)]
use std::ops::Add;
use std::process::Output;
use std::time::{Duration, Instant};
use std::usize;
use std::{
    fmt::Debug,
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

    //println!("{}",s1);

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let mut borrows_mutably = || list.push(7);

    // borrows_mutably();
    // println!("After calling closure: {list:?}");

    // #[derive(Debug)]
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // let p = Point { x: 0, y: 7 };
    // match p {
    //     Point { x: 0, y: 7 } => {
    //         println!("{:?} {:?}", p.x, p.y);
    //     }
    //     Point { x, y } => {
    //         println!("{}", x);
    //     }
    //     _ => todo!(),
    // }

    // enum Shape {
    //     Rectangle { width: i32, height: i32 },
    //     Circle(i32),
    // }
    // let shape = Shape::Circle(10);
    // match shape {
    //     Shape::Rectangle { width, height } => println!("{},{}", width, height),
    //     Shape::Circle(radius) => println!("{}", radius),
    // }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("{}, {}", first, last);
        }
    }

    struct SemVer(i32, i32, i32);
    let version = SemVer(1, 32, 2);
    match version {
        SemVer(major, _, _) => {
            println!("{}", major);
        }
    }
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
