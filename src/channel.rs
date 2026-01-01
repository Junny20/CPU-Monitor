use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<T>();
        Channel {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn split(self) -> (Sender<T>, Receiver<T>) {
        (self.sender, self.receiver)
    }
}
