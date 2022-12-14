use crate::import_content;
use std::str::FromStr;

pub fn clear_sections(){
    let contents = import_content::import("inputs/cleaning.txt");
    let split = contents.split('\n').collect::<Vec<&str>>();
    let mut sum = 0;
    for s in split.clone(){
        if clearing_pair_overlap_completely(s){
            sum += 1;
        }
    }
    println!("{}",sum);

    let mut sum = 0;
    for s in split.clone(){
        if clearing_pair_overlap_at_all(s){
            sum += 1;
        }
    }
    println!("{}",sum);
}

fn clearing_pair_overlap_at_all(s: &str) -> bool{
    let pair_split = s.split(',').collect::<Vec<&str>>();
    if pair_split.len() == 2 {
        let (min_0, max_0) = get_clearing_min_max(pair_split[0]);
        let (min_1, max_1) = get_clearing_min_max(pair_split[1]);
        // let mut does_overlapp = false;
        if min_0 < min_1{
            // check if 0 completely overlaps 1
            if max_0 >= min_1 {
                return true;
            }
        }
        
        if min_1 < min_0{
            // check if 1 completely overlaps 0
            if max_1 >= min_0 {
                return true;
            }
        }

        if min_0 == min_1 {
            return true;
        }
    }
    return false;
}

fn clearing_pair_overlap_completely(s: &str) -> bool{
    let pair_split = s.split(',').collect::<Vec<&str>>();
    if pair_split.len() == 2 {
        let (min_0, max_0) = get_clearing_min_max(pair_split[0]);
        let (min_1, max_1) = get_clearing_min_max(pair_split[1]);
        // println!("{}-{}, {}-{}", min_0, max_0, min_1, max_1);
        // let mut does_overlapp = false;
        if min_0 <= min_1{
            // check if 0 completely overlaps 1
            if max_0 >= max_1 {
                return true;
            }
        }
        
        if min_1 <= min_0{
            // check if 1 completely overlaps 0
            if max_1 >= max_0 {
                return true;
            }
        }
    }
    return false;
}

fn get_clearing_min_max(s: &str) -> (i32, i32){
    let string = s.trim();
    let range_split = string.split('-').collect::<Vec<&str>>();
    let min: i32 = FromStr::from_str(&range_split[0]).unwrap();
    let max: i32 = FromStr::from_str(range_split[1]).unwrap();
    return (min, max);
}