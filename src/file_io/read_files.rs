use std::{path::{Path, PathBuf}};

use crate::converters::converter::Converter;

use super::receiver::Receiver;

pub fn read_files<I, P, T>(pathes: I, _converter: Converter<T>) -> Receiver<T>
where
    I: Iterator<Item = P>,
    P: AsRef<Path>,
{
    

    let pathes : Vec<PathBuf> = pathes.map(|p| p.as_ref().to_path_buf()).collect();
    let (_sender, receiver) = async_channel::bounded(pathes.len());
    std::thread::spawn(move || {
        for _path in pathes.iter() {
            
        }
    });
    Receiver::new(receiver)
}
