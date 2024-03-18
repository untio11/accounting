use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Display},
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
/// Display the `u64` ID value as a hexadecimal string.
///
/// E.g.: `"5E8C0A84534B0F04"`
impl<Of: Identify> Debug for ID<Of> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}
impl<Of: Identify> Display for ID<Of> {
    /// Display the `u64` ID value as a hexadecimal string.
    ///
    /// E.g.: `"5E8C0A84534B0F04"`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Transfer Debug
    }
}

pub trait Identify
where
    Self: Hash + Debug,
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

    /// Discard `other`'s ID Type and rewrap it in `Self::IdType`.
    ///
    /// Nice to have transitive ID's when subtypes define their own
    /// implementation of `.id()`.
    ///
    /// # Example
    /// `Account` is a value in `Node::ProperAccount(Account)` that
    /// defines its own implementation of `Identify.id()``, though
    /// their ID types differ, so `Node` can easily pass through
    /// the id value without recomputing it.
    /// ```
    /// let account_id: ID<Account> = Account::default().id();
    /// let node_id: ID<Node> = Node::transfer_from(account_id);
    /// ```
    fn transfer_from<OtherIdType: Identify>(other: ID<OtherIdType>) -> ID<Self::IdType>
    where
        <Self as Identify>::IdType: Identify,
    {
        match other {
            ID(other_id_value, _) => ID(other_id_value, PhantomData),
        }
    }
}
