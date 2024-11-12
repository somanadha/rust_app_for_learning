use List::{Cons, Nil};
use Test::{One, Two};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    let enum_test = One;
    println!("{:?}", enum_test);
}
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
#[derive(Debug)]
enum Test {
    One,
    Two,
}
