use std::{
    collections::{HashSet, HashMap},
    error::Error,
    fmt::{Debug, Display},
    fs::File,
};

use identity_hash::BuildIdentityHasher;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct SillyStringError {
    msg: String,
}

impl SillyStringError {
    pub fn new(s: &impl ToString) -> Self {
        Self { msg: s.to_string() }
    }
}

impl Display for SillyStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("StringError")?;
        f.write_str(&self.msg)?;
        f.write_str(")")?;
        Ok(())
    }
}

impl Error for SillyStringError {}

impl From<String> for SillyStringError {
    fn from(value: String) -> Self {
        SillyStringError { msg: value }
    }
}

impl From<&str> for SillyStringError {
    fn from(value: &str) -> Self {
        SillyStringError {
            msg: value.to_owned(),
        }
    }
}

pub type FastHashSet<T> = HashSet<T, BuildIdentityHasher<T>>;
pub type FastHashMap<K,V> = HashMap<K, V, BuildIdentityHasher<K>>;

pub trait Loadable
where
    Self: Sized,
{
    fn load(file: &File) -> Result<Vec<Self>>;
}
