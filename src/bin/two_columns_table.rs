use indexmap::IndexMap;
use rust_app_for_learning::get_penguin_data_string;

fn main() {
    let two_columns_table: IndexMap<String, String> =
        get_two_columns_table_from_string(get_penguin_data_string());
    print_two_column_table(two_columns_table);
}

pub fn get_two_columns_table_from_string<T, U>(table_data_string: String) -> IndexMap<T, U>
where
    T: std::fmt::Debug,
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
    T: std::hash::Hash,
    T: Eq,
    U: std::fmt::Debug,
    U: std::str::FromStr,
    U::Err: std::fmt::Debug,
    T: std::hash::Hash,
    U: Eq,
{
    let table_data_as_lines = table_data_string.lines();

    let mut two_column_table: IndexMap<T, U> = IndexMap::new();

    for one_line in table_data_as_lines {
        if one_line.trim().is_empty() {
            continue;
        }

        let tokens_in_one_line: Vec<_> = one_line
            .split(',')
            .map(|one_token| one_token.trim())
            .collect();

        if tokens_in_one_line.len() == 2 {
            let column_1 = tokens_in_one_line[0].parse().unwrap();
            let column_2 = tokens_in_one_line[1].parse().unwrap();
            //println!("Column 1: {:?}, Column 2: {:?}", column_1, column_2);
            two_column_table.insert(column_1, column_2);
        }
    }
    two_column_table
}

pub fn print_two_column_table<T, U>(table: IndexMap<T, U>)
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    for one_row_in_table in table.iter() {
        println!("{:?} - {:?}", one_row_in_table.0, one_row_in_table.1)
    }
}
