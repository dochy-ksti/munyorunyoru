use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use crate::{error::munyo_error::PathItem, from_str_with_metabuilder, Error};
use serde::de::DeserializeOwned;
use shrink_pool::ShrinkPool;

use crate::builder::builder::{Builder, MetaBuilder};

use super::{Data, Receiver};

static IO_THREAD: OnceLock<ShrinkPool> = OnceLock::new();

fn io_thread() -> &'static ShrinkPool {
    IO_THREAD.get_or_init(|| ShrinkPool::new(1))
}

/// Read files in a thread and parse them in a thread pool concurrently.
/// The thread and the pool's threads are automatically destroyed when no tasks are assigned for them.
/// When a task is assigned after that, a new thread is spawned. It costs some, so you may want to
/// assign as much tasks as possible at once to avoid the respawn cost.
/// 
/// # Example
/// ```
/// 
/// ```
pub struct Concurrent {
    pool: Arc<ShrinkPool>,
}

impl Clone for Concurrent {
    /// clone Concurrent. The cloned Concurrent shares the thread pool with the original.
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl Concurrent {
    /// Create Concurrent with the thread-pool's size = num_cpus::get().
    ///
    /// This creates a thread pool. If multiple Concurrents have sufficient tasks,
    /// the sum of threads will surpass num_cpus.
    ///
    /// If you want to share the pool, use clone().
    ///
    /// IO thread is always shared.
    pub fn new() -> Self {
        Self::with_pool_size(num_cpus::get())
    }

    /// Create Concurrent with the thread-pool's size.
    ///
    /// If you want to share the pool, use clone().
    pub fn with_pool_size(pool_size: usize) -> Self {
        Self {
            pool: Arc::new(ShrinkPool::new(pool_size)),
        }
    }

    /// Read files and build items with meta_builder. This is not meant for general usage.
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

    /// Read files and deserialize items from them.
    ///
    /// Reading starts in the order given, and parsing and deserializing will follow.
    /// But it's a concurrent process and the items will be sent as soon as they are ready, so the order of finished items is unknown.
	/// 
	/// See [Concurrent] to know how to use this.
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

    fn inner<I, P, F, T>(&self, pathes: I, f: F) -> Receiver<Result<Data<T>, Error>>
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
                match crate::read_file(&path) {
                    Ok(s) => {
                        pool.execute(move || {
                            //the channel has sufficient size, so no blocking occurs.
                            sender.send_blocking(f((path, s))).ok();
                            //when receiver is dropped, sending fails. It's OK.
                        });
                    }
                    Err(e) => {
                        sender.send_blocking(Err(e)).ok();
                        return;
                    }
                }
            }
        });
        Receiver::new(receiver)
    }

    /// Do something in this thread pool. Maybe useful, maybe not.
    pub fn do_something_with_pool<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pool.execute(move || f())
    }

    /// Do something in the io thread. Maybe useful, maybe not.
    pub fn do_something_with_io_thread<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        io_thread().execute(move || f());
    }
}
