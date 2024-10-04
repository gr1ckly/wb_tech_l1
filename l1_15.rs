pub fn quicksort(arr: Vec<i32>) -> Vec<i32>{
    if arr.len() < 2{
        return arr;
    }
    let mut left_vec = Vec::new();
    let mut mid_vec = Vec::new();
    let mut right_vec = Vec::new();
    let pivot = arr[0];
    for elem in arr{
        if (elem < pivot){
            left_vec.push(elem);
        } else if (elem > pivot) {
            right_vec.push(elem);
        } else {
            mid_vec.push(elem);
        }
    }
    let mut new_vec = Vec::new();
    new_vec.extend(&quicksort(left_vec));
    new_vec.extend(&mid_vec);
    new_vec.extend(&quicksort(right_vec));
    new_vec
}