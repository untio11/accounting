use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub mod types;

pub trait Identify
where
    Self: Hash,
{
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }
}
