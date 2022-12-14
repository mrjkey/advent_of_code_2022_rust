use std::fs;

pub fn import(file_path: &str) -> String{
    // let file_path = "inputs/snacks.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}