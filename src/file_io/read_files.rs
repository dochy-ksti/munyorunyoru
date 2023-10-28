use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
};

use shrink_pool::ShrinkPool;

use crate::{
    builder::builder::{Builder, MetaBuilderArguments},
    error::{MunyoError, MunyoResult, ReadFileError},
    lang::process_file_text::process_file_text,
};

use super::receiver::Receiver;

pub fn read_files<I, P, T, B, MB>(
    pathes: I,
    meta_builder: MB,
) -> Receiver<Result<T, ReadFileError>>
where
    I: Iterator<Item = P>,
    P: AsRef<Path>,
    T: Send + 'static,
    MB: Fn(MetaBuilderArguments) -> B + Send + Sync + 'static,
    B: Builder<T>,
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
                                .send_blocking(Err(ReadFileError::Parse(path, e)))
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
