use std::sync::{Arc, Mutex, Weak};
use std::thread;

#[derive(Debug)]
struct College {
    name: String,
    age: u32,
    students: Mutex<Vec<Weak<Student>>>,
}

#[derive(Debug)]
struct Student {
    name: String,
    college: Arc<College>,
}

fn main() {
    let my_college = Arc::new(College {
        name: "My College".into(),
        age: 10,
        students: Mutex::new(Vec::new()),
    });

    let mut thread_handles = Vec::new();

    for i in 1..10 {
        let my_college = my_college.clone();
        let thread_handle = thread::spawn(move || {
            println!("Thread {} started.", i);
            let student = Arc::new(Student {
                name: format!("student-{}", i),
                college: my_college.clone(),
            });

            println!(
                "Student: {} is from the college: {}",
                student.name, student.college.name
            );

            let mut mutex_lock_college_reference = my_college.students.lock().unwrap();

            mutex_lock_college_reference.push(Arc::downgrade(&student));

            //println!("Thread {} ended.", i);
        });

        thread_handles.push(thread_handle);
    }

    println!("Original College:{:?}", my_college);

    for thred_handle in thread_handles {
        let _ = thred_handle.join();
    }
    for student in my_college.students.lock().unwrap().iter() {
        if let Some(student) = student.upgrade() {
            println!("College:{:?}", student.college);
        }
    }
}
