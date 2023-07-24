use std::thread;
use std::sync;
use tokio::io;

use crate::algorithm::Algorithm;

pub struct Worker {
  handle: thread::JoinHandle<io::Result<(String, String)>>,
  sender: sync::mpsc::Sender<()>
}

impl Worker {
  pub fn start(start: u64, incrementor: u64, algorithm: Algorithm) -> Worker {
    let (sender, receiver) = sync::mpsc::channel();j
    
    let handle = thread::spawn(move || {
      let mut nonce = start.to_string();

      loop {
        let hash = 
      }

      Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No nonce found"
      ))
    });
    
    Worker { handle, sender }
  }
}