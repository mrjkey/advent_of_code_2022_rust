use std::fs;

fn get_elf_calories(contents: String) -> Vec<i32>{
    let split = contents.split('\n');
    let mut elf_vec: Vec<i32> = Vec::new();
    let mut sum = 0;
    for s in split{
        if s == ""{
            elf_vec.push(sum);
            sum = 0;
        } else {
            let my_int: i32 = s.parse().unwrap();
            sum += my_int;
        }
    }
    return elf_vec;
}

pub fn print_elf_snacks() {
    let file_path = "inputs/snacks.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut elf_vec = get_elf_calories(contents);
    elf_vec.sort();
    elf_vec.reverse();
    // for elf in elf_vec{
    //     println!("{}", elf)
    // }
    println!("{}, {}, {}", elf_vec[0], elf_vec[1], elf_vec[2]);
    let sum = elf_vec[0] + elf_vec[1] + elf_vec[2];
    println!("{}", sum);
    // let max_iter = elf_vec.iter().max();
    // let biggest;
    // match max_iter {
    //     Some(i) => biggest = *i,
    //     None => biggest = 0
    // }
    println!("{ }", elf_vec.len());
    // println!("{}", biggest);
}