pub fn flip_words(line: String) -> String{
    let mut new_line = String::new();
    for word in line.split(" "){
        new_line.insert_str(0, &format!("{} ", word));
    }
    new_line = String::from(new_line.trim_end());
    new_line
}