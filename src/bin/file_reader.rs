use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let data = open_file_for_read_match();
    // let data = open_file_for_read_simple().expect("Error getting data");
    // let data = open_file_for_read_simple_chanining()?;
    // let data = open_file_for_read_one_liner()?;
    // println!("******* Here is the data *******");
    // println!("{data}");

    let user_name = read_username_from_file()?;
    println!("{user_name}");

    Ok(())
}

fn open_file_for_read_match() -> String {
    let mut file = match File::open("delete.txt") {
        Ok(file) => {
            println!("File is opened successfully");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("delete.txt") {
                Ok(file) => {
                    println!("File is created successfully");
                    file
                }
                Err(_error) => panic!("File doesn't exists and trying to create a new file fails"),
            },
            _other_error => panic!("Error opening the file"),
        },
    };
    let mut file_data_strnig = String::new();
    match file.read_to_string(&mut file_data_strnig) {
        Ok(size) => {
            println!("File is read {size} bytes successfully");
        }
        Err(_error) => {
            println!("Unable to read the file successfuly");
        }
    }

    file_data_strnig
}

fn open_file_for_read_simple() -> Result<String, Error> {
    let mut file = File::open("delete.txt")?;

    let mut file_data_strnig = String::new();
    file.read_to_string(&mut file_data_strnig)?;

    Ok(file_data_strnig)
}

fn open_file_for_read_simple_chanining() -> Result<String, Error> {
    let mut file_data_strnig = String::new();
    File::open("delete.txt")?.read_to_string(&mut file_data_strnig)?;

    Ok(file_data_strnig)
}

fn open_file_for_read_one_liner() -> Result<String, Error> {
    fs::read_to_string("delete.txt")
}

fn read_username_from_file() -> Result<String, Error> {
    let mut username_file = File::open("user_credentials.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
