use std::{path::{Path, PathBuf}};

use shrink_pool::ShrinkPool;

use crate::{converters::converter::Converter, error::{MunyoResult, MunyoError}};

use super::{receiver::Receiver, recv_error::ReadFilesError};

pub fn read_files<I, P, T>(pathes: I, _converter: Converter<T>) -> Receiver<Result<T, ReadFilesError>>
where
    I: Iterator<Item = P>,
    P: AsRef<Path>,
    T: Send + 'static,
{
    

    let pathes : Vec<PathBuf> = pathes.map(|p| p.as_ref().to_path_buf()).collect();
    let (sender, receiver) = async_channel::bounded(pathes.len());
    std::thread::spawn(move || {
        let pool = ShrinkPool::new(num_cpus::get());

        for path in pathes.iter() {
            match std::fs::read_to_string(path){
                Ok(s) =>{
                    pool.execute(move ||{
                        

                    });
                },
                Err(e) =>{ 
                    sender.send_blocking(Err(ReadFilesError::ReadFile(path.to_owned()))).expect("async_channel::send_blocking failed");
                    return;
                }
            }
        }
    });
    Receiver::new(receiver)
}
