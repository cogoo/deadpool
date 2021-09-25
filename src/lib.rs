use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
};

pub struct ThreadPool {
    _handles: Vec<thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];

        for _ in 0..num_threads {
            let clone = receiver.clone();
            let handle = thread::spawn(move || loop {
                let work = match clone.lock().unwrap().recv() {
                    Ok(work) => work,
                    Err(_) => break,
                };

                println!("Start");
                work();
                println!("End");
            });
            _handles.push(handle);
        }

        Self { _handles, sender }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        let delayed_work = || thread::sleep(std::time::Duration::from_secs(1));

        pool.execute(delayed_work);
        pool.execute(|| println!("Do some work"));
        pool.execute(delayed_work);
        pool.execute(|| println!("Hello from thread"));
    }
}
