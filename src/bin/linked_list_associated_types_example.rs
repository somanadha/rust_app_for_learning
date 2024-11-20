use std::{borrow::BorrowMut, cell::RefCell, fmt::Debug};

fn main() {
    let mut list = DataSorterList::new();
    list.push(5);
    list.push(2);
    list.push(8);
    list.push(1);
    list.push(8);
    list.push(3);
    // Print the list to verify correctness
    let mut current = &list.head;
    while let Some(ref boxed_node) = current {
        println!("{}", boxed_node.data);
        current = &boxed_node.next;
    }
}

type Next<T> = Option<Box<Node<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node<T>
where
    T: PartialOrd + Eq + Clone,
{
    data: T,
    next: Next<T>,
}

pub struct DataSorterList<T>
where
    T: PartialOrd + Eq + Clone + Debug,
{
    head: Next<T>,
}

impl<T> DataSorterList<T>
where
    T: PartialOrd + Eq + Clone + Debug,
{
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, data: T) {
        let mut current_node = &self.head;
        
        while let Some(unwrapped_current_node) = current_node {
            println!("current_node = {:?}", current_node);
            if data > unwrapped_current_node.data {
                current_node = &unwrapped_current_node.next;
            } else {
                break;
            }
        }

        if self.head.is_none(){
            self.head = Some(Box::new(Node {
                data,
                next: None,
            }));
        } else {
            current_node = &mut Some(Box::new(Node {
                data,
                next: current_node.clone().unwrap().next,
            }));
        }
    }
}
