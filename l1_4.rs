use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct spmc{
    pub workers_count: u32,
    receiver: Arc<Mutex<Receiver<String>>>,
    sender: Sender<String>
}

impl spmc {
    pub fn new(count_workers: u32) -> Self{
        let (sender, receiver) = std::sync::mpsc::channel();
        spmc {
            workers_count: count_workers,
            receiver: Arc::new(Mutex::new(receiver)),
            sender: sender
        }
    }

    pub fn launch(&self){
        for i in 0..self.workers_count{
            let receiver = Arc::clone(&self.receiver);
            thread::spawn(move ||{
                loop {
                    let receiver = receiver.lock().unwrap();
                    let res = receiver.try_recv();
                    match res{
                        Ok(message) => println!("{}", message),
                        Err(e) => break
                    }
                }
            });
        }
        let mut counter = 0;
        loop {
            let sender = &self.sender;
            sender.send(counter.to_string()).unwrap();
            counter += 1;
        }
    }
}