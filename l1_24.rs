use std::collections::HashSet;
use std::ops::Add;

pub fn check_unique(line: &String) -> bool{
    let mut set = HashSet::new();
    let new_line = line.clone().to_lowercase();
    for char in new_line.chars(){
        if !set.insert(char){
            return false;
        }
    }
    true
}