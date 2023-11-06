use std::io;

pub fn mirchecker_panic(){
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    println!("input_string: {}", input_string);
}