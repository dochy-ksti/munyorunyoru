
type AsyncReceiver<T> = async_channel::Receiver<T>;

pub struct Receiver<T> {
    receiver: AsyncReceiver<T>,
}

impl<T> Receiver<T> {
    pub(crate) fn new(receiver: AsyncReceiver<T>) -> Receiver<T> {
        Receiver { receiver }
    }
    pub fn recv_blocking(&self) -> crate::Result<T> {
        Ok(self.receiver.recv_blocking()?)
    }
    pub fn try_recv(&self) -> crate::Result<T> {
        Ok(self.receiver.try_recv()?)
    }
    pub async fn recv_async(&self) -> crate::Result<T> {
        Ok(self.receiver.recv().await?)
    }
}
