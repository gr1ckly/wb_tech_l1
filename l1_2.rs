use std::thread;
use std::thread::JoinHandle;

const N: usize = 15;

pub fn square_parallel(){
    let mut arr: [usize; N] = [0; N];
    for i in 0..N{
        arr[i] = i + 1;
    }

    let mut thread_vec: Vec<JoinHandle<()>> = Vec::new();
    for i in arr{
        let curr_thread = thread::spawn(move || println!("{}", &i * &i));
        thread_vec.push(curr_thread);
    }
    for handle in thread_vec{
        handle.join().unwrap();
    }
}