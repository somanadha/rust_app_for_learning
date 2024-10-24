use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut thread_handles = vec![];
    for _ in 1..=10 {
        let counter_arc_clone = Arc::clone(&counter);
        let thread_handle = thread::spawn(move || {
            let mut counter_arc_clone_ref = counter_arc_clone.lock().unwrap();
            *counter_arc_clone_ref += 1;
        });
        thread_handles.push(thread_handle);
    }

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    println!("{:?}", counter.lock().unwrap());
}
