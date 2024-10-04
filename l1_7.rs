use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use chrono::Local;
use tokio_util::sync::CancellationToken;

pub fn close_chanel(){
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(std::sync::Mutex::new(receiver));
    let mut threads = Vec::new();
    for i in 1..5 {
        let thread_receiver = receiver.clone();
        let thread = thread::spawn(move || {
            let id = i;
            let rcvr = thread_receiver.lock().unwrap();
            loop {
                let res = rcvr.recv();
                match res {
                    Ok(message) => println!("thread {} : {}", i, message),
                    Err(e) => {
                        println!("End thread");
                        break
                    }
                }
            }
        });
        threads.push(Option::Some(thread));
    }
    let start = Local::now();
    while (Local::now() - start).num_seconds() <= 10{
        sender.send("lol").unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    drop(sender);
    for mut thread in threads{
        match thread.take(){
            Some(trd) => trd.join().unwrap(),
            None => continue
        }
    }
}

pub async fn tokio_tasks(){
    let token = CancellationToken::new();
    let task_token = token.clone();
    let task = tokio::spawn(async move {
        let mut counter = 1;
        while !task_token.is_cancelled() {
            println!("{}", counter);
            counter += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        println!("End of task!");
    });

    tokio::time::sleep(Duration::from_secs(5)).await;

    token.cancel();
    task.await.unwrap();
}