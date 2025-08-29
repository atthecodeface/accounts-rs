//a Imports
use std::marker::PhantomData;

use super::{Idx, IndexedSlice};

//a IndexedVec
//tp IndexedVec
/// An [IndexedVec] is a Vec of items with an index
pub struct IndexedVec<I, T, const M: bool>
where
    I: Idx,
{
    array: Vec<T>,
    _phantom: PhantomData<fn(&I)>,
}

//ip Default for IndexedVec<I, T, M>
impl<I, T, const M: bool> std::default::Default for IndexedVec<I, T, M>
where
    I: Idx,
{
    fn default() -> Self {
        let array = vec![];
        Self {
            array,
            _phantom: PhantomData,
        }
    }
}

//ip Index<Handle> for IndexedVec
impl<I, T, const M: bool> std::ops::Index<I> for IndexedVec<I, T, M>
where
    I: Idx,
{
    type Output = T;
    fn index(&self, idx: I) -> &T {
        &self.array[idx.index()]
    }
}

//ip IndexMut<Handle> for mutable IndexedVec
impl<I, T> std::ops::IndexMut<I> for IndexedVec<I, T, true>
where
    I: Idx,
{
    fn index_mut(&mut self, idx: I) -> &mut T {
        &mut self.array[idx.index()]
    }
}

//ip IndexedVec (mutable and immutable)
impl<I, T, const M: bool> IndexedVec<I, T, M>
where
    I: Idx,
{
    //ap next_index
    /// Gives the next index that will be assigned when `push` is
    /// called.
    #[inline]
    pub fn next_index(&self) -> I {
        I::from_usize(self.array.len())
    }

    //ap as_slice
    /// Return an [IndexSlice] for the contents
    #[inline(always)]
    pub fn as_slice(&self) -> &IndexedSlice<I, [T], M> {
        IndexedSlice::new(&self.array)
    }

    //mp push
    /// Push a new item onto the vector, and return it's index.
    #[inline]
    pub fn push(&mut self, d: T) -> I {
        let index = self.next_index();
        self.array.push(d);
        index
    }

    //ap get
    /// Get a ref to the item at the provided index, or None for out of bounds.
    #[inline]
    pub fn get(&self, index: I) -> Option<&T> {
        self.as_slice().get(index)
    }

    //ap is_empty
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    /*
        //ap array
        pub fn array(&self) -> &[T] {
            &self.array
        }

        //mp get
        /// get
        pub fn get(&self, name: &N) -> Option<H> {
            self.index.get(name).copied()
        }

        //mp add
        /// Add
        pub fn add(&mut self, name: N, data: D) -> H {
            let handle = self.array.len().into();
            self.array.push(data);
            self.index.insert(name, handle);
            handle
        }
        //mp find_or_add
        /// Add
        pub fn find_or_add(&mut self, name: N, data: D) -> H {
            let Some(handle) = self.index.get(&name) else {
                return self.add(name, data);
            };
            *handle
    }
        */
}

//ip IndexedVec mutable
impl<I, T> IndexedVec<I, T, true>
where
    I: Idx,
{
    //ap as_mut_slice
    /// Return an [IndexSlice] for the contents
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut IndexedSlice<I, [T], true> {
        IndexedSlice::new_mut(&mut self.array)
    }

    //ap get_mut
    /// Get a ref to the item at the provided index, or None for out of bounds.
    #[inline]
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }

    // Could add... pop, insert,
}

//ip IntoIter for IndexedVec
impl<'a, I, T, const M: bool> std::iter::IntoIterator for &'a IndexedVec<I, T, M>
where
    I: Idx,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    // Required method
    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.array.iter()
    }
}

//ip AsRef<[T]> for IndexedVec
impl<I, T, const M: bool> AsRef<[T]> for IndexedVec<I, T, M>
where
    I: Idx,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.array
    }
}

//ip AsRef<IndexedSlice<I, [T]>> for IndexedVec
impl<I, T, const M: bool> AsRef<IndexedSlice<I, [T], M>> for IndexedVec<I, T, M>
where
    I: Idx,
{
    #[inline]
    fn as_ref(&self) -> &IndexedSlice<I, [T], M> {
        IndexedSlice::new(&self.array)
    }
}

//ip Deref for IndexedVec
impl<I, T, const M: bool> std::ops::Deref for IndexedVec<I, T, M>
where
    I: Idx,
{
    type Target = IndexedSlice<I, [T], M>;
    #[inline]
    fn deref(&self) -> &IndexedSlice<I, [T], M> {
        IndexedSlice::new(&self.array)
    }
}

//ip AsMut<IndexedSlice<I, [T]>> for IndexedVec
impl<I, T> AsMut<IndexedSlice<I, [T], true>> for IndexedVec<I, T, true>
where
    I: Idx,
{
    #[inline]
    fn as_mut(&mut self) -> &mut IndexedSlice<I, [T], true> {
        IndexedSlice::new_mut(&mut self.array)
    }
}

//ip DerefMut for IndexedVec mutable
impl<I, T> std::ops::DerefMut for IndexedVec<I, T, true>
where
    I: Idx,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut IndexedSlice<I, [T], true> {
        IndexedSlice::new_mut(&mut self.array)
    }
}
