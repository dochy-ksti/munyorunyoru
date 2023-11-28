use std::fmt::{Debug, Display};
use std::ops::Deref;

/// Pretty printable with Display trait.
pub struct Processed<T> {
    pub result: Vec<T>,
}

impl<T> Processed<T> {
    pub fn new(result: Vec<T>) -> Self {
        Self { result }
    }
}

impl<T: Display> Display for Processed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.result {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<T: Debug> Debug for Processed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.result {
            writeln!(f, "{:?}", item)?;
        }
        Ok(())
    }
}

impl<T: Clone> Clone for Processed<T> {
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
        }
    }
}

impl<T> Deref for Processed<T>{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}