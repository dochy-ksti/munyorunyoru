use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use shrink_pool::ShrinkPool;

use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::{ReadFileError, read_file_error::PathItem},
    lang::process_file_text::process_file_text,
};

use super::receiver::Receiver;

pub struct Data<T>{
	pub path : PathBuf,
	pub items : Vec<T>
}

pub fn read_files<I, P, T, B, MB>(
    pathes: I,
    meta_builder: MB,
) -> Receiver<Result<Data<T>, ReadFileError>>
where
    I: Iterator<Item = P>,
    P: AsRef<Path>,
    MB: MetaBuilder<Item=B> + Send + Sync + 'static,
	B : Builder<Item=T>,
	T : Send + 'static,
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
                    pool.execute(move || match process_file_text(s, builder.as_ref()) {
                        Ok(_) => {}
                        Err(e) => {
                            sender
                                .send_blocking(Err(ReadFileError::Parse(PathItem::new(Some(path)), e)))
                                .expect("async_channel::send_blocking failed");
                        }
                    });
                }
                Err(e) => {
                    sender
                        .send_blocking(Err(ReadFileError::ReadFile(
                            path.to_owned(),
                            format!("{e}"),
                        )))
                        .expect("async_channel::send_blocking failed");
                    return;
                }
            }
        }
    });
    Receiver::new(receiver)
}
