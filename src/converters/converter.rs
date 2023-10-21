use std::collections::HashMap;

pub(crate) struct Input{}

pub(crate) struct Converter<T>{
    map : HashMap<String,Box<dyn Fn(Input) -> T>>
}