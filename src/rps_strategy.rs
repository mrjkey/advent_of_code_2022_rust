use std::fs;

pub fn strategy(){
    let file_path = "inputs/rps.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let split = contents.split('\n');
    let mut sum = 0;
    for s in split{
        if s.to_string() != ""{
            // println!("{}", s);
            let os = translate_opponent(s.to_string());
            let ms = get_shape(s.to_string());
            // println!("{} {}", os, ms);
            let points = shape_points(ms) + game_points(os, ms);
            // println!("{}", points);
            sum += points;
        }
    }
    println!("{}", sum);
    // print!("{}", sum);
    // let mut game_vec = get_elf_calories(contents);
    // let mut score = 0;
    // for game in game_vec
}

fn get_shape(s: String) -> char{
    let os = translate_opponent(s.to_string());
    if s.contains("X"){
        return get_loss(os);
    } else if s.contains("Y"){
        return os;
    } else {
        return get_win(os);
    }
}

fn get_win(c: char) -> char{
    if c == 'r'{
        return 'p';
    } else if c == 'p'{
        return 's';
    } else{return 'r';}
}

fn get_loss(c: char) -> char{
    if c == 'r'{
        return 's';
    } else if c == 'p'{
        return 'r';
    } else{return 'p';}
}

fn translate_opponent(s: String) -> char{
    if s.contains("A"){
        return 'r';
    } else if s.contains("B"){
        return 'p';
    } else {
        return 's';
    }
}

// fn translate_mine(s: String) -> char{
//     if s.contains("X"){
//         return 'r';
//     } else if s.contains("Y"){
//         return 'p';
//     } else {
//         return 's';
//     }
// }

fn game_points(s1: char, s2: char) -> i32{
    if s1 == s2{
        return 3;
    } else if s1 == 'r' && s2 == 's'{
        return 0;
    } else if s1 == 'p' && s2 == 'r'{
        return 0;
    } else if s1 == 's' && s2 == 'p'{
        return 0;
    } else{
        return 6;
    }
}

fn shape_points(s: char) -> i32{
    if s == 'r'{
        return 1;
    } else if s == 'p' {
        return 2;
    } else {
        return 3;
    }
}