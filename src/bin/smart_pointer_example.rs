use List::{Next, Nil};
use Test::{One, Two};

fn main() {
    let list = Next(1, Box::new(Next(2, Box::new(Next(3, Box::new(Nil))))));
    println!("{:?}", list);
    let enum_test = One;
    println!("{:?}", enum_test);
}
#[derive(Debug)]
enum List<T> {
    Next(T, Box<List<T>>),
    Nil,
}
#[derive(Debug)]
enum Test {
    One,
    Two,
}
