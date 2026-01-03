//! Channel module.
//!
//! Provides a generic Channel wrapper around mpsc channels for type-safe communication.

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

/// Generic channel structure.
/// 
/// Wraps a sender and receiver pair for a given type T.
pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    /// Creates a new Channel.
    /// 
    /// * Returns
    /// A new Channel instance with sender and receiver
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<T>();
        Channel {
            sender: tx,
            receiver: rx,
        }
    }

    /// Splits the channel into sender and receiver.
    /// 
    /// Consumes the Channel and returns the sender and receiver separately.
    /// 
    /// * Returns
    /// A tuple of (Sender<T>, Receiver<T>)
    pub fn split(self) -> (Sender<T>, Receiver<T>) {
        (self.sender, self.receiver)
    }
}
