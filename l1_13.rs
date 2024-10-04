use std::fmt::format;
use regex::Regex;

pub fn check_equals(){
    let mut full_line = String::new();
    let mut line = String::new();
    let input = std::io::stdin();
    while input.read_line(&mut line).unwrap() != 0{
        full_line.push_str(&line);
        line.clear();
    }
    for i in full_line.split("\n").filter(|&x| !x.is_empty()){
        let re = Regex::new(&format!(r"(^|\n){}($|\n)", regex::escape(i))).unwrap();
        if re.find_iter(&full_line).count() == 1{
            println!("{}", i);
        }
    }
}