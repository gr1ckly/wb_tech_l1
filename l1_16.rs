use std::collections::hash_map::Values;

//Принимает отсортированный массив и возвращает индекс указанного элемента
//Если элемента в массиве нет, то возвращает -1
pub fn bin_search(arr: Vec<i32>, value: i32) -> i32{
    let mut left_pointer = 0;
    let mut right_pointer = arr.len();
    while left_pointer != right_pointer{
        let mid = (right_pointer + left_pointer) / 2;
        if arr[mid] == value{
            return mid as i32;
        } else if value < arr[mid]{
            right_pointer = mid;
        } else {
            left_pointer = mid + 1;
        }
    }
    -1
}