use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, Thread, ThreadId};

pub struct safety_spmc{
    pub workers_count: u32,
    receiver: Arc<Mutex<Receiver<String>>>,
    sender: Option<Sender<String>>,
    //Каждый поток оборачивается в Option, чтобы, если он был уже завершен и была попытка дождаться конца его выполнения, можно было завершить программу без ошибки
    threads: Vec<Option<JoinHandle<()>>>
}

impl safety_spmc {
    pub fn new(count_workers: u32) -> Self{
        let (sender, receiver) = std::sync::mpsc::channel();
        safety_spmc {
            workers_count: count_workers,
            receiver: Arc::new(Mutex::new(receiver)),
            sender: Option::Some(sender),
            threads: Vec::new()
        }
    }

    pub fn launch(&mut self){
        for i in 0..self.workers_count{
            let new_receiver = Arc::clone(&self.receiver);
            let thread = thread::spawn(move ||{
                loop {
                    let mut_receiver = new_receiver.lock().unwrap();
                    let res = mut_receiver.try_recv();
                    //Если возникает ошибка связанная с получением данных из канала(закрытие одной стороны канала или др.), поток безопасно прекращает свою работу
                    match res{
                        Ok(message) => println!("{}", message),
                        Err(e) => break
                    }
                }
            });
            &self.threads.push(Option::Some(thread));
        }
        let mut counter = 0;
        loop {
            let link_sender = &self.sender;
            match link_sender {
                Some(sendr) => {
                    let result = sendr.send(counter.to_string());
                    match result{
                        Ok(()) => counter += 1,
                        Err(e) => break
                    }
                },
                None => break
            }
        }
    }
}

impl Drop for safety_spmc{
    fn drop(&mut self) {
         //Удаление sender для прекращения отправки сообщений и побуждения потоков закончить свое выполнение
        drop(self.sender.take());
        for _ in 0..self.threads.len(){
            let worker = self.threads.pop().take().unwrap();
            if let Some(thread) = worker {
                thread.join().unwrap();
            }
        }
    }
}