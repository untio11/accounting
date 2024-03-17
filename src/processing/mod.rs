use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub mod summaries;
pub mod types;

#[derive(PartialEq, Eq, Hash)]
pub struct ID<Of: Identify>(u64, PhantomData<Of>);

impl<Of: Identify> Into<String> for ID<Of> {
    fn into(self) -> String {
        format!("{:X}<{:?}>", self.0, self.1)
    }
}

impl<Of: Identify> Debug for ID<Of> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}<{:?}>", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum IdGroups {
    Transaction,
    Node,
}
pub trait Identify
where
    Self: Hash,
{
    type IdType;
    /// Generates a (somewhat?) consistent hash ID of self.
    /// Consistency is achieved by utilizing a fresh DefaultHasher
    /// for generating the hash.
    ///
    /// This claimed consistency has not been tested extensively
    /// at all lol.
    ///
    /// To implement id for your own type:
    /// ```
    /// fn id(&self) -> ID<Phantom> {
    ///     let mut hasher = DefaultHasher::new();
    ///     // Hash the relevant properties of self:
    ///     self.identifying_prop.hash(&mut hasher);
    ///     // ...
    ///     return ID(hasher.finish(), PhantomData);
    /// }
    /// ```
    fn id(&self) -> ID<Self::IdType>
    where
        <Self as Identify>::IdType: Identify,
    {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return ID(hasher.finish(), PhantomData);
    }
}
