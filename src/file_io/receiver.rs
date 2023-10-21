
use crate::error::MunyoResult;

type AsyncReceiver<T> = async_channel::Receiver<T>;

pub(crate) struct Receiver<T>{
    receiver : AsyncReceiver<T>
}

impl<T> Receiver<T>{
    pub(crate) fn new(receiver : AsyncReceiver<T>) -> Receiver<T>{
        Receiver { receiver }
    }
    pub fn recv(&self) -> MunyoResult<T>{
        Ok(self.receiver.recv_blocking()?)
    }
    pub fn try_recv(&self) -> MunyoResult<T>{
        Ok(self.receiver.try_recv()?)
    }
    pub async fn recv_async(&self) -> MunyoResult<T>{
        Ok(self.receiver.recv().await?)
    }
}