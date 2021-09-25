use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
};

pub struct ThreadPool {
    _handles: Vec<thread::JoinHandle<()>>,
    sender: Sender<Box<dyn FnMut() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn FnMut() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];

        for _ in 0..num_threads {
            let clone = receiver.clone();
            let handle = thread::spawn(move || loop {
                let mut work = match clone.lock().unwrap().recv() {
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

    pub fn execute<T: FnMut() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn it_works() {
        let n = AtomicU32::new(0);
        let nref = Arc::new(n);
        let pool = ThreadPool::new(10);
        let delayed_work = || thread::sleep(std::time::Duration::from_secs(1));
        let clone = nref.clone();
        let complex_operation = move || {
            clone.fetch_add(1, Ordering::SeqCst);
        };

        pool.execute(delayed_work);
        pool.execute(|| println!("Do some work"));
        pool.execute(delayed_work);
        pool.execute(|| println!("Hello from thread"));
        pool.execute(complex_operation.clone());
        pool.execute(complex_operation.clone());
        pool.execute(complex_operation.clone());

        thread::sleep(std::time::Duration::from_secs(2));

        assert_eq!(nref.load(Ordering::SeqCst), 3);
    }
}
