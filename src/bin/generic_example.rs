fn main() {
    //finding_largest_generic();
    //calculate_distance_from_origin();
    check_tuple_2_mixup();
}

fn finding_largest_generic() {
    let integer_vector = vec![22, 3, 88, 0, -2, 34, 89];
    let largest = get_largest(&integer_vector);
    println!(
        "Largest of the following vector is {} at the index positon {}",
        largest.1, largest.0
    );
    println!("{:?}", integer_vector);

    let string_vector = vec!["somanadha", "udaya", "Srikar", "Srikrishna"];
    let largest = get_largest(&string_vector);
    println!(
        "Largest of the following vector is \"{}\" at the index positon {}",
        largest.1, largest.0
    );
    println!("{:?}", string_vector);
}

fn get_largest<T: PartialOrd + Copy>(data: &Vec<T>) -> (u32, T) {
    let mut larges_value = data[0];
    let mut largest_index = 0;

    for (value, index) in data.iter().zip(0..) {
        if larges_value < *value {
            larges_value = *value;
            largest_index = index;
        }
    }

    (largest_index, larges_value)
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn get_y(&self) -> &f32 {
        &self.y
    }
    fn get_distance_from_origin(&self) -> f32 {
        (self.get_x().powi(2) + self.get_y().powi(2)).sqrt()
    }
}

fn calculate_distance_from_origin() {
    let point = Point { x: 5.0, y: 6.0 };
    println!("{:#?}", point.get_distance_from_origin());
}

#[derive(Debug)]
struct Tuple2<T, U> {
    item_1: T,
    item_2: U,
}
impl<T, U> Tuple2<T, U> {
    fn tuple_2_mixup<V, W>(self, another: Tuple2<V, W>) -> Tuple2<T, W> {
        Tuple2 {
            item_1: self.item_1,
            item_2: another.item_2,
        }
    }
}

fn check_tuple_2_mixup() {
    let t1 = Tuple2 {
        item_1: 10,
        item_2: 20,
    };

    let t2 = Tuple2 {
        item_1: "Satya",
        item_2: "Udaya",
    };

    let t3 = t1.tuple_2_mixup(t2);

    println!("{:#?}", t3);
}
