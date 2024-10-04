use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use chrono::Local;

pub fn running_n_time(n: i64){
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let read_receiver = receiver.clone();
    let read_thread = thread::spawn(move ||{
        let recv = read_receiver.lock().unwrap();
        loop{
            let res = recv.recv();
            match res{
                Ok(ans) => continue,
                Err(e) => {
                    println!("End thread");
                    break
                }
            }
        }
    });
    let start = Local::now();
    while (Local::now() - start).num_seconds() < n{
        let res = sender.send(Local::now() - start).unwrap();
    }
    drop(sender);
    drop(receiver);
}