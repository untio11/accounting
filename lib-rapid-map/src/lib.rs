#![allow(dead_code)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    rc::Rc,
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
        ID(hasher.finish(), PhantomData)
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

use chrono::NaiveDate;

pub trait WithDate {
    fn date(&self) -> NaiveDate;
}

pub enum DateBoundary<T: WithDate> {
    First(T),
    Last(T),
}

#[derive(Clone)]
struct DateSorted<T: WithDate + Ord>(Vec<T>);

impl<T: WithDate + Ord> DateSorted<T> {
    fn parse(mut input: Vec<T>) -> Self {
        input.sort_unstable();
        Self(input)
    }
}

struct RapidMap<'a, T: Identify + WithDate + Ord + Eq> {
    data: DateSorted<T>,
    /// Map from a ID of T to a pointer that points at the entry in the underlying
    /// vector. Allow for constant time indexing like a normal hashmap.
    id_index: HashMap<ID<T>, Rc<&'a T>>,
    /// Map from a date (day/month/year) to the first and last T within that date.
    /// Returns None if there are 0 T's with that date.
    /// TODO: Date boundaries should be iterators starting at those points.
    date_index: HashMap<NaiveDate, Option<DateBoundary<T>>>,
}
impl<'a> RapidMap<'a, TestData<'a>> {
    fn new(data: Vec<TestData<'a>>) -> Self {
        // Take ownership of data
        // Sort on date
        // Build index?
        let data: DateSorted<TestData<'a>> = DateSorted::parse(data);
        let id_index = Self::build_id_index(&data);
        let date_index = Self::build_date_index(&data);
        Self {
            data,
            id_index,
            date_index,
        }
    }

    fn build_id_index(
        data: &'a DateSorted<TestData<'a>>,
    ) -> HashMap<ID<TestData<'a>>, Rc<&'a TestData<'a>>> {
        let mut index = HashMap::new();
        for t in &data.0 {
            index.insert(t.id(), Rc::new(t));
        }
        index
    }
    fn build_date_index(
        data: &DateSorted<TestData<'a>>,
    ) -> HashMap<NaiveDate, Option<DateBoundary<TestData<'a>>>> {
        let index = HashMap::new();
        index
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{Identify, RapidMap, WithDate};
    use chrono::NaiveDate;

    #[derive(Clone, Copy, PartialEq, Hash, Eq, Debug, PartialOrd, Ord)]
    pub struct TestData<'a>(&'a str);
    impl<'a> WithDate for TestData<'a> {
        fn date(&self) -> NaiveDate {
            NaiveDate::parse_from_str(self.0, "%Y%m").unwrap()
        }
    }
    impl<'a> Identify for TestData<'a> {
        type IdType = TestData<'a>;
    }
    const DATA: [TestData; 3] = [
        TestData("2024-01"),
        TestData("2024-03"),
        TestData("2024-02"),
    ];
    #[test]
    fn basic_creation() {
        let t = RapidMap::new(Vec::from(DATA));
    }
}