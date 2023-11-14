use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use crate::{error::munyo_error::PathItem, from_str_with_metabuilder, Error};
use serde::de::DeserializeOwned;
use shrink_pool::ShrinkPool;

use crate::builder::builder::{Builder, MetaBuilder};

use super::{data::Data, receiver::Receiver};

static IO_THREAD: OnceLock<ShrinkPool> = OnceLock::new();

fn io_thread() -> &'static ShrinkPool {
    IO_THREAD.get_or_init(|| ShrinkPool::new(1))
}

/// Read files with a thread and parse them with a thread pool concurrently.
/// The thread and the pool's threads are automatically destroyed when no tasks are assigned for them.
/// When a task is assigned after that, a new thread is spawned. It costs some, so you may want to
/// assign as much tasks as possible at once to avoid the respawn-cost.
pub struct Concurrent {
    pool: Arc<ShrinkPool>,
}

impl Concurrent {
    /// Create this with the thread-pool's size.
    pub fn new(num_cpus: usize) -> Self {
        Self {
            pool: Arc::new(ShrinkPool::new(num_cpus)),
        }
    }
    pub fn read_files_with_builder<I, P, T, B, MB>(
        &self,
        pathes: I,
        meta_builder: MB,
    ) -> Receiver<Result<Data<T>, Error>>
    where
        I: Iterator<Item = P>,
        P: AsRef<Path>,
        MB: MetaBuilder<Item = B> + Send + Sync + 'static,
        B: Builder<Item = T>,
        T: Send + 'static,
    {
        self.inner(pathes, move |(path, s)| {
            match from_str_with_metabuilder(s.as_str(), &meta_builder) {
                Ok(v) => Ok(Data::new(path, v.result)),
                Err(e) => Err(Error::Parse(PathItem::new(Some(path)), e)),
            }
        })
    }

    pub fn deserialize_files<I, P, T>(&self, pathes: I) -> Receiver<Result<Data<T>, Error>>
    where
        I: Iterator<Item = P>,
        P: AsRef<Path>,
        T: Send + 'static + DeserializeOwned,
    {
        self.inner(pathes, move |(path, s)| match crate::from_str(s.as_str()) {
            Ok(v) => Ok(Data::new(path, v)),
            Err(e) => Err(e),
        })
    }

    pub fn inner<I, P, F, T>(&self, pathes: I, f: F) -> Receiver<Result<Data<T>, Error>>
    where
        I: Iterator<Item = P>,
        P: AsRef<Path>,
        F: Fn((PathBuf, String)) -> Result<Data<T>, Error> + Send + Sync + 'static,
        T: Send + 'static,
    {
        let pathes: Vec<PathBuf> = pathes
            .into_iter()
            .map(|p| p.as_ref().to_path_buf())
            .collect();

        let (sender, receiver) = async_channel::bounded(pathes.len());
        let f = Arc::new(f);
        let pool = self.pool.clone();
        io_thread().execute(move || {
            for path in pathes.into_iter() {
                let sender = sender.clone();
                let f = f.clone();
                let pool = pool.clone();
                match std::fs::read_to_string(&path) {
                    Ok(s) => {
                        pool.execute(move || {
                            sender.send_blocking(f((path, s))).expect("sender failed")
                        });
                    }
                    Err(e) => {
                        sender
                            .send_blocking(Err(Error::ReadFile(path, format!("{e}"))))
                            .expect("async_channel::send_blocking failed");
                        return;
                    }
                }
            }
        });
        Receiver::new(receiver)
    }
}
