use crate::import_content;

pub fn communication(){
    let contents = import_content::import("inputs/signals.txt");
    start_of_packet(contents.clone());
    start_of_message(contents.clone());
}

fn start_of_message(contents: String){
    let mut signals = contents.chars().into_iter();
    let mut index = 1;
    let mut c: Vec<Option<char>> = Vec::new();
    for _ in 0..14 {
        c.push(signals.next());
        index += 1;
        // println!("{}", x);
    }
    if vec_unique(c.clone()) {
        println!("{}", index);
    } else {
        loop {
            c.remove(0);
            c.push(signals.next());
            if vec_unique(c.clone()){
                // for _ in 0..c.len() {
                //     match c.remove(0) {
                //         Some(i) => println!("{}", i),
                //         None => println!("empty")
                //     }
                // }
                break;
            }
            index += 1;
            // if index % 100 == 0 {
            //     println!("{}", c.len());
            // }
            // break;
        }
        println!("{}", index);
    }
}

fn vec_unique(c: Vec<Option<char>>) -> bool {
    for i in 0..c.len()-1 {
        for j in i+1..c.len(){
            if c[i] == c[j] {
                return false;
            }
        }
    }
    return true;
}

fn start_of_packet(contents: String){
    let mut signals = contents.chars().into_iter();
    let mut index = 4;
    let mut c = [signals.next(),signals.next(),signals.next(),signals.next()];
    if !not_unique(c){
        println!("{}",index);
    } else {
        loop {
            c = shift(c, signals.next());
            index += 1;
            if !not_unique(c){
                break;
            }
            // break;
        }
        println!("{}", index);
    }
}

fn not_unique(c: [Option<char>; 4]) -> bool {
    let b1 = c[0] == c[1] || c[0] == c[2] || c[0] == c[3];
    let b2 = c[1] == c[2] || c[1] == c[3];
    let b3 = c[2] == c[3] || b1 || b2;
    return b3;
}

fn shift(mut c: [Option<char>; 4], next_c: Option<char>) -> [Option<char>; 4]{
    c[0] = c[1];
    c[1] = c[2];
    c[2] = c[3];
    c[3] = next_c;
    return c;
}