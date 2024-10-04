pub fn remove_i<T>(mut vec: Vec<T>, i: usize) -> Vec<T>{
    if i <= vec.len() && i > 0{
        vec.remove(i - 1);
    }
    vec
}