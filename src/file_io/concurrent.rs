use std::{sync::OnceLock, path::Path};

use anyhow::Error;
use shrink_pool::ShrinkPool;

use crate::builder::builder::{MetaBuilder, Builder};

use super::{read_files::Data, receiver::Receiver};


static IO_THREAD : OnceLock<ShrinkPool> = OnceLock::new();

fn io_thread() -> &'static ShrinkPool{
	IO_THREAD.get_or_init(|| ShrinkPool::new(1))
}

pub struct Concurrent{
	pool : ShrinkPool
}

impl Concurrent{
	pub fn new(num_cpus : usize) -> Self{ Self{ pool : ShrinkPool::new(num_cpus) }}
	pub fn read_files<I, P, T, B, MB>(pathes: I, meta_builder: MB) -> Receiver<Result<Data<T>, Error>>
	where
		I: Iterator<Item = P>,
		P: AsRef<Path>,
		MB: MetaBuilder<Item = B> + Send + Sync + 'static,
		B: Builder<Item = T>,
		T: Send + 'static,
	{
		todo!()
	}	
}

pub fn read_files<I, P, T, B, MB>(pathes: I, meta_builder: MB) -> Receiver<Result<Data<T>, Error>>
where
    I: Iterator<Item = P>,
    P: AsRef<Path>,
    MB: MetaBuilder<Item = B> + Send + Sync + 'static,
    B: Builder<Item = T>,
    T: Send + 'static,
{
    let pathes: Vec<PathBuf> = pathes.map(|p| p.as_ref().to_path_buf()).collect();
    let (sender, receiver) = async_channel::bounded(pathes.len());
    std::thread::spawn(move || {
        let pool = ShrinkPool::new(num_cpus::get());
        let builder = Arc::new(meta_builder);

        for path in pathes.into_iter() {
            let sender = sender.clone();
            let builder = builder.clone();
            match std::fs::read_to_string(&path) {
                Ok(s) => {
                    pool.execute(
                        move || match from_str_with_metabuilder(&s, builder.as_ref()) {
                            Ok(_) => {}
                            Err(e) => {
                                sender
                                    .send_blocking(Err(Error::Parse(PathItem::new(Some(path)), e)))
                                    .expect("async_channel::send_blocking failed");
                            }
                        },
                    );
                }
                Err(e) => {
                    sender
                        .send_blocking(Err(Error::ReadFile(path.to_owned(), format!("{e}"))))
                        .expect("async_channel::send_blocking failed");
                    return;
                }
            }
        }
    });
    Receiver::new(receiver)
}