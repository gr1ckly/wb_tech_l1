use std::sync::mpsc;
use std::thread;

const N: usize = 10;

pub fn square_sum(){
    let mut arr: [usize; N] = [0; N];
    let mut res = 0;
    for i in 0..N{
        arr[i] = i + 1;
    }
    let (tx, rx) = mpsc::channel();

    let mut thread_vec = vec![];
    for i in arr{
        let txi = tx.clone();
        let curr_thread = thread::spawn(move || txi.send(&i * &i));
        thread_vec.push(curr_thread);
    }
    for handle in thread_vec{
        handle.join().unwrap();
    }
    drop(tx);
    for received in rx{
        res += received;
    }
    println!("{}", res);
}