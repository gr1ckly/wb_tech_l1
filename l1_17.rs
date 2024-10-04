use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn concurrent_increments(){
    let counter = Arc::new(Mutex::new(0));
    let mut thread_vec = Vec::new();
    for _ in 0..5{
        let new_counter = counter.clone();
        let thread = thread::spawn(move || {
            let mut countr = new_counter.lock().unwrap();
            *countr += 1;
        });
        thread_vec.push(Option::Some(thread));
    }
    thread::sleep(Duration::from_secs(2));
    for handle in thread_vec{
        match handle{
            Some(handle) => handle.join().unwrap(),
            None => continue
        }
    }
    println!("{}", counter.lock().unwrap());
}