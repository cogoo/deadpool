use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

pub struct ThreadPool {
    _handles: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];

        for _ in 0..num_threads {
            let clone = receiver.clone();
            let handle = thread::spawn(move || loop {
                let work = clone.lock().unwrap().recv().unwrap();
                work();
            });
            _handles.push(handle);
        }
        Self { _handles }
    }

    pub fn execute<T: Fn()>(&self, work: T) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        pool.execute(|| println!("Do some work"));
    }
}
