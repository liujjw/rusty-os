#![no_std]

#[cfg(test)]
mod tests;

use core::slice;
use core::iter::IntoIterator;
use core::ops::{Deref, DerefMut};

/// A contiguous array type backed by a slice.
///
/// `StackVec`'s functionality is similar to that of `std::Vec`. You can `push`
/// and `pop` and iterate over the vector. Unlike `Vec`, however, `StackVec`
/// requires no memory allocation as it is backed by a user-supplied slice. As a
/// result, `StackVec`'s capacity is _bounded_ by the user-supplied slice. This
/// results in `push` being fallible: if `push` is called when the vector is
/// full, an `Err` is returned.
#[derive(Debug)]
pub struct StackVec<'a, T: 'a> {
    storage: &'a mut [T],
    len: usize
}

impl<'a, T: 'a> StackVec<'a, T> {
    /// Constructs a new, empty `StackVec<T>` using `storage` as the backing
    /// store. The returned `StackVec` will be able to hold `storage.len()`
    /// values.
    pub fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        return StackVec {
            storage: storage,
            len: 0usize
        };
    }

    /// Constructs a new `StackVec<T>` using `storage` as the backing store. The
    /// first `len` elements of `storage` are treated as if they were `push`ed
    /// onto `self.` The returned `StackVec` will be able to hold a total of
    /// `storage.len()` values.
    ///
    /// # Panics
    ///
    /// Panics if `len > storage.len()`.
    pub fn with_len(storage: &'a mut [T], len: usize) -> StackVec<'a, T> {
        let storage_len = storage.len();
        if len > storage_len {
            panic!("len > storage.len()");
        }
        return StackVec {
            storage: storage,
            len: len
        };
    }

    /// Returns the number of elements this vector can hold.
    pub fn capacity(&self) -> usize {
       self.storage.len()
    }

    /// Shortens the vector, keeping the first `len` elements. If `len` is
    /// greater than the vector's current length, this has no effect. Note that
    /// this method has no effect on the capacity of the vector.
    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            self.len = len;
        }
    }

    /// Extracts a slice containing the entire vector, consuming `self`.
    ///
    /// Note that the returned slice's length will be the length of this vector,
    /// _not_ the length of the original backing storage.
    pub fn into_slice(self) -> &'a mut [T] {
        return self.storage;
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        return &(self.storage);
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        return &mut (self.storage);
    }

    /// Returns the number of elements in the vector, also referred to as its
    /// 'length'.
    pub fn len(&self) -> usize {
        return self.len;
    }

    /// Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    /// Returns true if the vector is at capacity.
    pub fn is_full(&self) -> bool {
        return self.len == self.storage.len();
    }

    /// Appends `value` to the back of this vector if the vector is not full.
    ///
    /// # Error
    ///
    /// If this vector is full, an `Err` is returned. Otherwise, `Ok` is
    /// returned.
    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if (&*self).is_full() {
            return Err(());
        } else {
            self.storage[self.len] = value;
            self.len += 1;
            return Ok(());
        }
    }
}

impl<'a, T: Clone + 'a> StackVec<'a, T> {
    /// If this vector is not empty, removes the last element from this vector
    /// by cloning it and returns it. Otherwise returns `None`.
    pub fn pop(&mut self) -> Option<T> {
        if (&*self).is_empty() {
            return None;
        } else {
            let res = Some(self.storage[self.len - 1].clone());
            self.len -= 1;
            return res;
        }
    }
}

// FIXME: Implement `Deref`, `DerefMut`, and `IntoIterator` for `StackVec`.
impl<'a, T> Deref for StackVec<'a, T> {
    type Target = &'a mut [T];
    
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<'a, T> DerefMut for StackVec<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

impl<'a, T: 'a + Copy> IntoIterator for StackVec<'a, T> {
    type Item = T;
    type IntoIter = StackVecIntoIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        StackVecIntoIter {
            stackVec: self,
            idx: 0
        }
    }
}

pub struct StackVecIntoIter<'a, T> {
    stackVec: StackVec<'a, T>,
    idx: usize
}

impl<'a, T: 'a + Copy> core::iter::Iterator for StackVecIntoIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.idx >= self.stackVec.len {
            return None;
        } else {
            let result = self.stackVec.storage[self.idx];
            self.idx += 1;
            return Some(result);
        }
    }
}

// FIXME: Implement IntoIterator` for `&StackVec`.
// impl<'a, T: 'a + Copy> IntoIterator for &StackVec<'a, T> {
//     type Item = T;
//     type IntoIter = RefStackVecIntoIter<'b,'a, T>;
    
//     fn into_iter(self) -> Self::IntoIter {
//         RefStackVecIntoIter {
//             stackVec: self,
//             idx: 0
//         }
//     }
// }

// pub struct RefStackVecIntoIter<'b,'a, T> {
//     stackVec: &'b StackVec<'a, T>,
//     idx: usize
// }

// impl<'b,'a, T: 'a + Copy> core::iter::Iterator for RefStackVecIntoIter<'b,'a, T> {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {
//         if self.idx >= self.stackVec.len {
//             return None;
//         } else {
//             let result = self.stackVec.storage[self.idx];
//             self.idx += 1;
//             return Some(result);
//         }
//     }
// }