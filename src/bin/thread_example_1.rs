use std::{thread, time::Duration};

fn main() {
    let my_vector: Vec<i32> = (1..=26).collect();

    let join_handler_1 = thread::spawn(|| {
        println!("{:?}", &my_vector);
        for item in my_vector {
            println!("In the spawn-1 thread count is {:?}", item);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let join_handler_2 = thread::spawn(|| {
        for alphabet in 'A'..='Z' {
            println!("In the spawn-2 thread alphabet is {alphabet}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    join_handler_1.join().unwrap();
    join_handler_2.join().unwrap();
}
