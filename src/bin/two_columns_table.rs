use std::{collections::HashMap, fmt::Debug};

fn main() {}

pub fn get_two_columns_table_from_String<T, U>(table_data_string: &str) -> HashMap<T, U>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
    T: std::hash::Hash,
    T: Eq,
    U: std::str::FromStr,
    U::Err: std::fmt::Debug,
    T: std::hash::Hash,
    U: Eq,
{
    let table_data_as_lines = table_data_string.lines();

    let mut two_column_table: HashMap<T, U> = HashMap::new();

    for (line_index, one_line) in table_data_as_lines.enumerate() {
        if line_index == 0 || one_line.trim().is_empty() {
            continue;
        }

        let tokens_in_one_line: Vec<_> = one_line
            .split(',')
            .map(|one_token| one_token.trim())
            .collect();

        if tokens_in_one_line.len() == 2 {
            let column_1 = tokens_in_one_line[0].parse().unwrap();
            let column_2 = tokens_in_one_line[1].parse().unwrap();
            two_column_table.insert(column_1, column_2);
        }
    }
    two_column_table
}

pub fn print_two_column_table<T, U>(table: HashMap<T, U>)
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    for one_row_in_table in table {
        println!("{:?} - {:?}", one_row_in_table.0, one_row_in_table.1)
    }
}
