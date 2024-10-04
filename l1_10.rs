use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

const N: usize = 3;

pub fn conveyor(){
    let (sender_1, receiver_1) = mpsc::channel();
    let (sender_2, receiver_2) = mpsc::channel();
    let mut arr:[u32; N] = [0; N];
    for i in 0..N{
        arr[i] = i as u32 + 1;
    }
    let receiver_1 = Arc::new(Mutex::new(receiver_1));
    let sender_2 = Arc::new(Mutex::new(sender_2));
    let receiver_2 = Arc::new(Mutex::new(receiver_2));
    let first_receiver = receiver_1.clone();
    let second_sender = sender_2.clone();
    let second_receiver = receiver_2.clone();
    let first_thread = thread::spawn(move || {
        loop {
            let recv = first_receiver.lock().unwrap();
            let sendr = second_sender.lock().unwrap();
            let mut counter = 0;
            while counter < N {
                match recv.recv() {
                    Ok(ans) => {
                        sendr.send(ans * ans).unwrap();
                        counter += 1;
                    },
                    Err(e) => {
                        drop(recv);
                        drop(sendr);
                        break
                    }
                }
            }
        }
    });
    let second_thread = thread::spawn(move ||{
        let rcv = second_receiver.lock().unwrap();
        let mut counter = 0;
        while counter < N {
            match rcv.recv() {
                Ok(ans) => {
                    println!("{}", ans);
                    counter += 1;
                }
                Err(e) => {
                    break
                }
            }
        }
    });

    for i in 0..arr.len(){
        sender_1.send(arr[i]).unwrap();
        thread::sleep(Duration::from_millis(10));
    }
    second_thread.join().unwrap();
    drop(sender_1);
    drop(sender_2);
    drop(receiver_1);
    drop(receiver_2);
}