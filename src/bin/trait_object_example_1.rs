use std::iter;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        todo!()
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        todo!()
    }
}
