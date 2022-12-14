use std::fs;

pub fn item_priorities(){
    let file_path = "inputs/items.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let split = contents.split('\n');
    let mut sum = 0;
    for s in split.clone(){
        if s.to_string() != ""{
            let (s1, s2) = s.split_at(s.len()/2);
            // println!("{} and {}", s1.len(), s2.len());
            // let mut s1_vec: Vec<char> = s1.chars().collect();
            let s1_ints: Vec<i32> = s_to_sorted_int_vec(s1);
            let s2_ints: Vec<i32> = s_to_sorted_int_vec(s2);

            let prio = find_matched_priority(s1_ints, s2_ints);
            sum += prio;
            // for s1_c in s1.chars(){
            //     s1_ints.push(letter_priority(s1_c));
            // }
            // s1_ints.sort();
            // // s1_sorted.sort_by(|a, b| b.cmp(a));
            // // let s1_returned: String = s1_sorted.into_iter().collect();
            // println!("{}", prio);
        }
    }
    println!("{}", sum);
    // let mut group_index = 0;
    // let mut count = 0;
    let mut group: Vec<&str> = Vec::new();
    let mut sum = 0;
    for s in split.clone(){
        if s.to_string() != ""{
            group.push(s);
            if group.len() >= 3{
                let s1 = s_to_sorted_int_vec(group[0]);
                let s2 = s_to_sorted_int_vec(group[1]);
                let s3 = s_to_sorted_int_vec(group[2]);
                let prio = find_3_matched_priority(s1, s2, s3);
                group = Vec::new();
                sum += prio;
            }
        }
    }
    println!("{}", sum);


}

fn letter_priority(c: char) -> i32{
    let mut value = c as i32;
    if value >= 97{
        value = value - 96;
    }else{
        value = value - 64 + 26;
    }
    // println!("{}", value);
    return value;
}

fn s_to_sorted_int_vec(s: &str) -> Vec<i32>{
    let mut s_ints: Vec<i32> = Vec::new();
    for s_c in s.chars(){
        s_ints.push(letter_priority(s_c));
    }
    s_ints.sort();
    return s_ints;
}

fn find_matched_priority(i1: Vec<i32>, i2: Vec<i32>) -> i32{
    let mut i1_iter = i1.into_iter();
    let mut i2_iter = i2.into_iter();
    let mut i1_val = i1_iter.next();
    let mut i2_val = i2_iter.next();
    // let mut count: i32 = 0;
    loop {
        // count += 1;
        if i1_val < i2_val{
            i1_val = i1_iter.next();
        } else if i1_val > i2_val{
            i2_val = i2_iter.next();
        } else {
            match i1_val{
                Some(i) => return i,
                None => return 0
            }
        }
    }
    // return 0;
}

fn find_3_matched_priority(i1: Vec<i32>, i2: Vec<i32>, i3: Vec<i32>) -> i32{
    let mut i1_iter = i1.into_iter();
    let mut i2_iter = i2.into_iter();
    let mut i3_iter = i3.into_iter();
    let mut i1_val = i1_iter.next();
    let mut i2_val = i2_iter.next();
    let mut i3_val = i3_iter.next();
    loop {
        if i1_val < i2_val || i1_val < i3_val{
            i1_val = i1_iter.next();
        } else if i2_val < i1_val || i2_val < i3_val{
            i2_val = i2_iter.next();
        } else if i3_val < i1_val || i3_val < i2_val{
            i3_val = i3_iter.next();
        } else {
            match i1_val{
                Some(i) => return i,
                None => return 0
            }
        }
    }
    // return 0;
}