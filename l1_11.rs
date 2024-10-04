use std::collections::HashMap;
use std::ptr::null;

pub fn merge_in_intervals(mut weather: Vec<f64>) -> HashMap<[i64; 2], Vec<f64>>{
    let mut map = HashMap::new();
    weather.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut start = weather[0];
    let mut start = start.round() as i64;
    let mut start = (((start + 1) / 10) - 1) * 10;
    let mut prev_start = start;
    let mut curr_vec = Vec::new();
    for i in 0..weather.len(){
        if weather[i] >= start as f64 && weather[i] < (start + 10) as f64{
            curr_vec.push(weather[i]);
        } else {
            while weather[i] >= (start + 10) as f64 {
                start += 10;
            }
            if !curr_vec.is_empty(){
                let ins_vec = curr_vec.clone();
                map.insert([prev_start, prev_start + 10], ins_vec);
                curr_vec.clear();
                curr_vec.push(weather[i]);
                prev_start = start;
            }
        }
    }
    if !curr_vec.is_empty(){
        map.insert([prev_start, prev_start + 10], curr_vec);
    }
    map
}