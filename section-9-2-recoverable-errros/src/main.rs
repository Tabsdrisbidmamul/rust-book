use std::{fs::File, io, io::Read, io::Write};

fn main() {
    let mut file = File::create("hello.txt")
        .unwrap_or_else(|error| panic!("Problem creating the file {:?}", error));

    let buffer = String::from("Hello World");

    let bytes: &usize = &file.write(&buffer.as_bytes()).unwrap();

    println!("written bytes {} ", bytes);

    // let read_file = File::open("hello.txt").expect("file should be included in repo");

    // println!("file has been opened {:?}", read_file);

    let res = read_username_from_file_op().expect("hello.txt not found");

    let lines = last_char_of_first_line(&res);

    println!("res:\n {}", res);
    println!("lines:\n {:?}", lines);
}

// fn read_username_from_file_match() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file_op() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// need to add fs to crates package manager
// fn read_username_from_file_std() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
