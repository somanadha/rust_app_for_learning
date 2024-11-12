use std::{fmt::Display, ops::Add};

fn main() {
    let point_1 = Point { x: 10, y: 20 };
    let point_2 = Point { x: 20, y: 10 };

    println!("point_1 = {:#?}, point_2={:#?} ", point_1, point_2);
    println!("point_1 + point_2 = {:#?}", point_1 + point_2);

    let mm = MilliMeters(1000.0);
    let m = Meters(1.0);
    let mm_plus_m = mm.clone() + m.clone();
    let m_plus_mm = m.clone() + mm.clone();

    println!(
        "mm = {:?}, m = {:?} and mm_plus_m = {:?} millimeters",
        mm.0, m.0, mm_plus_m.0
    );
    println!(
        "m = {:?}, mm = {:?} and m_plus_mm = {:?} meters",
        m.0, mm.0, m_plus_mm.0
    );
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Meters(f32);

#[derive(Debug, Clone, Copy)]
struct MilliMeters(f32);

impl Add<Meters> for MilliMeters {
    type Output = MilliMeters;

    fn add(self, other: Meters) -> Self::Output {
        MilliMeters(self.0 + 1000.0 * other.0)
    }
}

impl Add<MilliMeters> for Meters {
    type Output = Meters;

    fn add(self, other: MilliMeters) -> Self::Output {
        Meters(self.0 + other.0 / 1000.0)
    }
}
