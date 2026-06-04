use std::{sync::{mpsc,Arc, Mutex}, thread};


pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Self {
        let thread  = thread::spawn(move || {
            // receiver;
            
            loop{
            
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    },
                    Err(_)=> { 
                        println!("Worker id {id} disconnected");
                        break;
                    }
                }
            }
        });
        Worker {id, thread}
    }
}

pub struct ThreadPool {
    workers : Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }

    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let mut receiver = Arc::new(Mutex::new(receiver));



        for id in 0..size {
            // Debugging
            println!("Worker is getting a reciver and a job is going to be executed");
            //DOUBT:  Does it mean that workers push didn't work because we didn't had a job ???
            workers.push(Worker::new(id, Arc::clone(&receiver)));
            println!("Workers have been pushed with the reciver");
        }
        // Debugging
        println!("Threadpool with the sender job is sent");
        //DOUBT: But i have a doubt what kind of sender we are sending like what does sender contain cause job hasn't been executed right?
        ThreadPool {workers, sender}
    }  

    pub fn execute<F>(&self, val :F)  
        where 
            F: FnOnce() + Send + 'static
        {
            let job = Box::new(val);
            self.sender.send(job).unwrap();
    }
    
}