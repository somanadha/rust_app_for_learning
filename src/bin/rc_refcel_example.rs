use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct College {
    name: String,
    age: u32,
    students: RefCell<Vec<Weak<Student>>>,
}

#[derive(Debug)]
struct Student {
    name: String,
    college: Rc<College>,
}

// About the need & usage for Rc wrapper
//======================================
//
// Due to strict ownership rules that Rust imposes, to support many-to-one relationship
// (i.e multiple objects sharing the same object as a member), Reference Counting
// (std::rc::Rc) wrapper needs to be used over the ONE "server" object that needs to be
// shared across MANY "client" objects.
//
// What Rc does, is that it would keep the server object intact and adds a wrapper that
// takes care of keeping a count for number of client objects that have the server object
// referencing. Once the count reaches zero then server gets garbage collected.
//
// In the above struct definitions, it is required that, for multiple students to share the
// same college object. So in the "Student" struct, "College" object is shared through "Rc".
// In this case college object is the server object and student object is the client object.
//
// Similar to "Student" struct, "College" struct also contains multiple Student objects
// in a Vector, NOT warapped through "Rc" but wrapped through "Weak".

// About the need & usage for Weak wrapper
//=========================================
//
// Weak wrapper is to avoid reference cycles and prevent memory leaks in scenarios where,
// two or more Rc pointers might end up referencing each other. (Ex: Trees, 2-way Lists, Graphs)
//
// Student should be a weak reference in the College struct, as Student also has the College
// as a reference leading to Cyclic references.
//
// Alos, since, a Student can't exist without a college and if College goes ways then all
// students should also go away, Student should be made a Weak Rc (i.e. "Weak") in the College
// struct where as the College should be a Strong RC (i.e. plain "Rc") in the Student struct.
//
// To make things more clear "Rc" (std::rc::Rc) is the Strong type and "Weak" (std::rc::Weak)
//  is the  weak type equivalent for Rc - i.e. Reference Counting.
//
// In order to store a Student object as a Weak (Rc) type, "downgrade" associated function
// needs to be called on "Rc" type, that converts the underlying object to a "Weak" object.
//
// However to access the fields of that Weak type, it should be upgraded to a Strong type.
// For converting a "Weak" type to strong "Rc" type "upgrade" needs to be called.
//
// It is not possible to get the details from a Weak object, so it has to be upgraded by
// calling "upgrade" method on the Weak type. It returns an Option object of Rc. So "unwrap"
// method needs to be called after calling "upgrade". Before calling "unwrap" it is highly
// recommended to check the availability of the Weak reference through "match" or "if-let"
// as the Weak object might go out of life by the time "upgrade" is called.
//
// Rc is not thread safe. In multi-threaded scenario std::sync::Arc needs to be used. Similar
// to std::rc:Weak thare is another thread safe version of std::sync::Weak which is the
// counter part of it string type std::sync::Arc

// About the need & usage for RefCel
//==================================
//
// RefCell is is a powerful tool that allows for interior mutability—meaning that data can be
// mutated even when RefCell is wrapping an immutable reference This is particularly useful in
// scenarios where Rust's usual borrowing rules needs to be bypassed  at runtime.
// (ex: Graphs, Caching)
//
// The "borrow_mut" method on a RefCell allows to have read/write access to the wrapped in object
// The "borrow" method on a RefCell allows to have read only access to the wrapped in object
//
// RefCell is not thread safe. In multi-threaded scenario Mutex needs to be used

fn main() {
    let mut my_college = Rc::from(College {
        name: "My College".into(),
        age: 10,
        students: RefCell::new(Vec::new()),
    });

    let mut student_vec = Vec::new();

    for i in 1..=10 {
        let student = Rc::from(Student {
            name: format!("student-{}", i),
            college: my_college.clone(),
        });

        my_college
            .students
            .borrow_mut()
            .push(Rc::downgrade(&student));

        println!("Student Inside Scope: {:?}", student);

        student_vec.push(student);
    }

    for student in my_college.students.borrow().iter() {
        if let Some(student) = student.upgrade() {
            println!("Student Outside Scope: {:?}", student);
        }
    }
}
