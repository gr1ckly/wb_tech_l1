pub fn flip_line(line: String) -> String{
    let mut new_line = String::new();
    for symbol in line.chars(){
        new_line.insert(0, symbol);
    }
    new_line
}