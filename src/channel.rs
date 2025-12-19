use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub struct Channel<T> {
    sender: Sender<T>, 
    receiver: Receiver<T>
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<T>();
        Channel { sender: tx, receiver: rx}
    }

    pub fn split(self) -> (Sender<T>, Receiver<T>) {
        (self.sender, self.receiver)
    }
}