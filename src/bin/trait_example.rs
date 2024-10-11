use std::fmt::Debug;

fn main() {
    test_measurements();
}

fn test_measurements() {
    let s = Square { side: 10.0 };

    let r = Rectangle {
        length: 10.0,
        width: 20.0,
    };

    let t = Triangle {
        base: 5.0,
        height: 6.0,
        side1: 7.0,
        side2: 8.0,
    };

    print_measurements(&s);
    print_measurements(&r);
    print_measurements(&t);
}

fn print_measurements<T>(measurements: &T)
where
    T: Measurements,
{
    measurements.summerize();
}

trait Measurements {
    fn find_area(&self) -> f64;
    fn find_circumference(&self) -> f64;
    fn get_mesurements(&self) -> String;
    fn summerize(&self) {
        println!(
            "{} : with Circumference {} and Area {}",
            self.get_mesurements(),
            self.find_circumference(),
            self.find_area()
        );
    }
}

#[derive(Debug)]
struct Square {
    side: f64,
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64,
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
    side1: f64,
    side2: f64,
}

impl Measurements for Square {
    fn find_area(&self) -> f64 {
        self.side * self.side
    }

    fn find_circumference(&self) -> f64 {
        4.0 * self.side
    }

    fn get_mesurements(&self) -> String {
        format!("{:?}", self)
    }
}

impl Measurements for Rectangle {
    fn find_area(&self) -> f64 {
        self.length * self.width
    }

    fn find_circumference(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
    fn get_mesurements(&self) -> String {
        format!("{:?}", self)
    }
}

impl Measurements for Triangle {
    fn find_area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn find_circumference(&self) -> f64 {
        self.base + self.side1 + self.side2
    }
    fn get_mesurements(&self) -> String {
        format!("{:?}", self)
    }
}
