//a Imports
use std::collections::HashMap;
use std::marker::PhantomData;

use super::{Idx, IndexedVec};

//a IndexKey
//tt IndexKey
pub trait IndexKey<'key>: Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'key {}

//it IndexKey
impl<'key, T> IndexKey<'key> for T where
    T: Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'key
{
}

//a VecWithIndex
//tp VecWithIndex
/// An [VecWithIndex] is an IndexedVec of items with an array index,
/// and a dictionary mapping an index key to array indices
///
/// Once an element is added to the VecWithIndex it cannot be mutated
/// or removed; any array index returned by methods is valid for the
/// lifetime of the VecWithIndex.
///
/// The index is a mapping from key to an index to the internal array;
/// references to entries can either be by using a reference to an
/// index key, or by using an array index.
///
/// It suports Deref into the array; it thus exposes 'keys' and
/// 'contains' methods of the underlying HashMap explicitly.
pub struct VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
{
    array: IndexedVec<I, D, false>,
    index: HashMap<K, I>,
    phantom: PhantomData<&'vwi fn()>,
}

//ip Default for VecWithIndex<K, I, D>
impl<'vwi, K, I, D> std::default::Default for VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
{
    fn default() -> Self {
        let array = IndexedVec::default();
        let index = HashMap::default();
        Self {
            array,
            index,
            phantom: PhantomData,
        }
    }
}

//ip Debug for VecWithIndex<K, I, D>
impl<'vwi, K, I, D> std::fmt::Debug for VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
    D: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "VecWInd[")?;
        let mut first = true;
        for (_k, h) in self.index.iter() {
            if !first {
                fmt.write_str(", ")?;
            }
            let d = &self.array[*h];
            write!(fmt, "{h:?}: {d:?}")?;
            first = false;
        }
        write!(fmt, "]")
    }
}

//ip Index<ArrayIndex> for VecWithIndex
impl<'vwi, K, I, D> std::ops::Index<I> for VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
{
    type Output = D;
    fn index(&self, n: I) -> &D {
        &self.array[n]
    }
}

//ip IntoIter for VecWithIndex
impl<'iter, 'vwi, K, I, D> std::iter::IntoIterator for &'iter VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
{
    type Item = &'iter D;
    type IntoIter = std::slice::Iter<'iter, D>;

    // Required method
    fn into_iter(self) -> std::slice::Iter<'iter, D> {
        self.array.into_iter()
    }
}

//ip Deref for VecWithIndex
impl<'vwi, K, I, D> std::ops::Deref for VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
{
    type Target = IndexedVec<I, D, false>;
    #[inline]
    fn deref(&self) -> &IndexedVec<I, D, false> {
        &self.array
    }
}

//ip AsRef<[D]> for VecWithIndex
impl<'vwi, K, I, D> AsRef<[D]> for VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
{
    #[inline]
    fn as_ref(&self) -> &[D] {
        self.array.as_ref()
    }
}

//ip VecWithIndex
impl<'vwi, K, I, D> VecWithIndex<'vwi, K, I, D>
where
    K: IndexKey<'vwi>,
    I: Idx + 'vwi,
{
    //mp find_key
    /// Find the key in the index, if it is there
    pub fn find_key(&self, key: &K) -> Option<I> {
        self.index.get(key).copied()
    }

    //mp find_or_add
    /// Add data to the array and index, but only if it is not present
    ///
    /// Returns (found, index) - i.e. found is true if the key was
    /// already present, and false if the key was not present and the
    /// data was added to the array
    pub fn find_or_add<F: FnOnce(&K) -> D>(&mut self, key: K, f: F) -> (bool, I) {
        if let Some(index) = self.index.get(&key) {
            (true, *index)
        } else {
            let index = self.array.push(f(&key));
            self.index.insert(key, index);
            (false, index)
        }
    }

    //mp insert
    /// Add data to the array and index, but only if it is not present
    ///
    /// If it is already present, return an Err
    pub fn insert<F: FnOnce(&K) -> D>(&mut self, key: K, f: F) -> Result<I, I> {
        if let Some(index) = self.index.get(&key) {
            Err(*index)
        } else {
            let index = self.array.push(f(&key));
            self.index.insert(key, index);
            Ok(index)
        }
    }

    //mp keys
    /// Iterate through the keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.index.keys()
    }

    //mp contains
    /// Returns true if this contains a key
    pub fn contains(&self, key: &K) -> bool {
        self.index.contains_key(key)
    }
}
