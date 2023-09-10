use std::fs::File;
use std::io::{ self, Read };
// use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    println!("{:?}", greeting_file_result);

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => panic!("The file was not found"),
//             other_error => panic!("Problem opening the file: {:?}", error),
//         },
//     };

//     let greeting_file = greeting_file_result.unwrap();
    let greeting_file = greeting_file_result.expect("Oh no");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // std::fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
