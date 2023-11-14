pub(crate) mod concurrent;
use std::path::PathBuf;

/// File's path and parsed items.
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
    receiver: AsyncReceiver<T>,
}


impl<T> Receiver<T> {
    pub(crate) fn new(receiver: AsyncReceiver<T>) -> Receiver<T> {
        Receiver { receiver }
    }
	/// Wait for the item to be ready and receive it.
    pub fn recv_blocking(&self) -> crate::Result<T> {
        Ok(self.receiver.recv_blocking()?)
    }
	/// Receive an item if it's ready, or an error if it's not.
    pub fn try_recv(&self) -> crate::Result<T> {
        Ok(self.receiver.try_recv()?)
    }
	/// Receive an item asynchronously.
	/// 
	/// This only uses std::thread and async_channel, so I believe this async fn is runtime agnostic.
    pub async fn recv_async(&self) -> crate::Result<T> {
        Ok(self.receiver.recv().await?)
    }
}
