use std::collections::HashMap;
use std::sync::{Arc, LockResult, Mutex};
use std::thread;
use dashmap::DashMap;
use crate::l1_7::close_chanel;

pub fn write_HashMap(){
    let mut map = HashMap::new();
    let map = Arc::new(Mutex::new(map));
    let mut threads = Vec::new();
    for i in 0..5{
        let new_map = map.clone();
        let thread = thread::spawn(move || {
            new_map.lock().unwrap().insert(i.to_string(), i);
        });
        threads.push(Some(thread));
    }

    for i in threads{
        if let Some(join_handle) = i{
            join_handle.join().unwrap();
        }
    }

    let map = map.lock().unwrap();
    for mut key in map.keys(){
        println!("{}: {}", key, map.get(key).unwrap());
    }
}

pub fn write_DashMap(){
    let map = Arc::new(DashMap::new());
    let mut threads = Vec::new();
    for i in 0..5{
        let new_map = map.clone();
        let thread = thread::spawn(move || {
            new_map.insert(i.clone().to_string(), i.clone());
        });
        threads.push(Some(thread));
    }

    for thread in threads{
        if let Some(join_handle) = thread{
            join_handle.join().unwrap();
        }
    }

    for entry in map.iter(){
        println!("{}: {}", entry.key(), entry.value());
    }
}