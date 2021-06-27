use std::net::TcpListener;
use std::io::prelude::*;
use crate::request::Request;
use crate::response::Response;

use threadpool::ThreadPool;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};

type ServerJob = Box<dyn FnOnce() + Send + 'static>;

pub struct TCPServer {
    listener: TcpListener,
    sender: Sender<ServerJob>,
}

impl TCPServer {
    pub fn new(port: &str, n_workers: usize, n_jobs: usize) -> TCPServer {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        let pool = ThreadPool::new(n_workers);

        let (tx, rx): (Sender<ServerJob>, Receiver<ServerJob>) = channel();

        let receiver = Arc::new(Mutex::new(rx));

        for _ in 0..n_jobs {
            let rx = Arc::clone(&receiver);
            pool.execute(move || loop {
                let job = rx.lock().unwrap().recv().unwrap();

                println!("Worker got a job; executing.");

                job();
            });
        }

        TCPServer { listener, sender: tx }
    }

    pub fn listen<T>(&self, listener: T)
        where
            T: FnOnce(Request, Response) + Copy + Send + Sync + 'static,
    {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();

            self.execute(move || {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                let raw_request = String::from_utf8_lossy(&buffer[..]);
                let raw_request_split: Vec<&str> = raw_request.split("\u{0}").collect();
                let content = raw_request_split[0];


                let response = Response::new(stream);
                let request = Request::from_str(content).unwrap();
                listener(request, response);
            });
        }
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}