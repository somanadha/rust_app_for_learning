
fn main() {
}

pub struct Node<T> 
where T : PartialOrd
{
    data : T,
    next: Next<T>,
}

type Next<T> = Option<Box<Node<T>>>;

pub struct DataSorterList<T> 
where T : PartialOrd
{
    head: Next<T>,
}

impl<T> DataSorterList<T> where T : PartialOrd {
    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn push(&mut self, data : T) {

        match &self.head {
            Some(node)=>  {
                let head_item = self.head.as_mut().unwrap();
                let current_item = head_item;
                let previous_item ;
                loop {
                    if (data > current_item.data){
                        previous_item = current_item;
                        current_item = current_item.next.as_mut().unwrap();

                    } else {
                        previous_item.next = Some(Box::new(Node {data : data, next : Some(current_item))}));
                        break;
                    }
                }
            }
            None => { // First time case
                let data_item = Box::new(Node {data : data, next : None});
                self.head = Some(data_item);
            }
        }

    }
    
}