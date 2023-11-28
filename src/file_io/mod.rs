pub(crate) mod concurrent;
use std::path::PathBuf;

use async_channel::TryRecvError;

/// File's path and parsed items.
#[derive(Debug, Clone, PartialEq)]
pub struct Data<T> {
    pub path: PathBuf,
    pub items: Vec<T>,
}

impl<T> Data<T> {
    pub fn new(path: PathBuf, items: Vec<T>) -> Self { Self { path, items } }
}

type AsyncReceiver<T> = async_channel::Receiver<T>;

/// Finished items will be sent over this Receiver asynchronously.
pub struct Receiver<T> {
    pub receiver: AsyncReceiver<T>,
}

impl<T> Receiver<T> {
    pub(crate) fn new(receiver: AsyncReceiver<T>) -> Receiver<T> {
        Receiver { receiver }
    }
	/// Wait for the item to be ready and receive it.
    /// Return None if all items are already received.
    pub fn recv_blocking(&self) -> Option<T> {
        self.receiver.recv_blocking().ok()
        
    }
	/// Receive an item if it's ready. Return None if it's not ready.
    /// Return Some(None) if all items are already received.
    pub fn try_recv(&self) -> Option<Option<T>> {
        match self.receiver.try_recv(){
            Ok(t) => Some(Some(t)),
            Err(TryRecvError::Closed) => Some(None),
            Err(TryRecvError::Empty) => None,
        }
    }
	/// Receive an item asynchronously. Return None if all items are already received.
	/// 
	/// This only uses std::thread and async_channel, so I believe this async fn is runtime agnostic.
    pub async fn recv_async(&self) -> Option<T> {
        self.receiver.recv().await.ok()
    }
}
