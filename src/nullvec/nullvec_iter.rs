// use std::iter::{IntoIterator, ExactSizeIterator, FusedIterator, TrustedLen};
use std::iter::{IntoIterator, ExactSizeIterator};

use super::NullVec;
use nullable::Nullable;
use traits::Slicer;

impl<T: Clone> NullVec<T> {
    pub fn iter_raw<'s>(&'s self) -> NullVecRawIter<'s, T> {
        NullVecRawIter {
            data: &self,
            current: 0,
        }
    }

    pub fn iter_not_null<'s>(&'s self) -> NullVecNotNullIter<'s, T> {
        NullVecNotNullIter {
            data: &self,
            current: 0,
        }
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Iterator returns Nullable
/// /////////////////////////////////////////////////////////////////////////////

impl<T: Clone> IntoIterator for NullVec<T> {
    type Item = Nullable<T>;
    type IntoIter = NullVecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        NullVecIntoIter {
            data: self,
            current: 0,
        }
    }
}

pub struct NullVecIntoIter<T: Clone> {
    data: NullVec<T>,
    current: usize,
}

impl<T: Clone> Iterator for NullVecIntoIter<T> {
    type Item = Nullable<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Nullable<T>>;
        if self.current < self.data.len() {
            result = unsafe { Some(self.data.iloc_unchecked(&self.current)) };
            // current must not be incremented after exhausted
            self.current += 1;
        } else {
            result = None;
        };
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.data.len() - self.current;
        (hint, Some(hint))
    }
}

// impl<T: Clone> ç for NullVecIntoIter<T> {
// fn is_empty(&self) -> bool {
// self.current == self.data.len()
// }
// }
//
// impl<T: Clone> FusedIterator for NullVecIntoIter<T> {}
//
// unsafe impl<T: Clone> TrustedLen for NullVecIntoIter<T> {}
//

impl<T: Clone> Clone for NullVecIntoIter<T> {
    fn clone(&self) -> Self {
        NullVecIntoIter {
            data: self.data.clone(),
            current: self.current,
        }
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Iterator returns raw values
/// /////////////////////////////////////////////////////////////////////////////

pub struct NullVecRawIter<'a, T: 'a + Clone> {
    data: &'a NullVec<T>,
    current: usize,
}

impl<'a, T: 'a + Clone> Iterator for NullVecRawIter<'a, T> {
    type Item = (bool, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<(bool, &T)>;
        if self.current < self.data.len() {
            let val: &T = unsafe { self.data.data.get_unchecked(self.current) };
            match self.data.mask {
                Some(ref mask) => {
                    result = Some((unsafe { *mask.get_unchecked(self.current) }, val));
                }
                None => {
                    result = Some((false, val));
                }
            };
            self.current += 1;
        } else {
            result = None;
        }
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.data.len() - self.current;
        (hint, Some(hint))
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Iterator returns non-null raw values
/// /////////////////////////////////////////////////////////////////////////////

pub struct NullVecNotNullIter<'a, T: 'a + Clone> {
    data: &'a NullVec<T>,
    current: usize,
}

impl<'a, T: 'a + Clone> Iterator for NullVecNotNullIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<&T>;
        if self.current < self.data.len() {
            // ToDo: consider perf
            match self.data.mask {
                Some(ref mask) => {
                    while (unsafe { *mask.get_unchecked(self.current) } == true) &
                          (self.current < self.data.len()) {
                        self.current += 1;
                    }
                    if self.current >= self.data.len() {
                        result = None;
                    } else {
                        let val: &T = unsafe { self.data.data.get_unchecked(self.current) };
                        result = Some(val);
                    }
                }
                None => {
                    let val: &T = unsafe { self.data.data.get_unchecked(self.current) };
                    result = Some(val);
                }
            };
            self.current += 1;
        } else {
            result = None;
        }
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // ToDo: pre-compute number of null using self.data.mask?
        let hint = self.data.len() - self.current;
        (hint, Some(hint))
    }
}

#[cfg(test)]
mod tests {

    use std::f64;
    use std::iter::IntoIterator;

    use nullable::Nullable;
    use nullvec::NullVec;
    use traits::VecBase;

    #[test]
    fn test_into_iter() {

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        let mut it = nvec.into_iter();
        assert_eq!(it.size_hint(), (3, Some(3)));

        assert_eq!(it.next(), Some(Nullable::Null));
        assert_eq!(it.size_hint(), (2, Some(2)));

        assert_eq!(it.next(), Some(Nullable::Value(2)));
        assert_eq!(it.size_hint(), (1, Some(1)));

        assert_eq!(it.next(), Some(Nullable::Value(3)));
        assert_eq!(it.size_hint(), (0, Some(0)));

        // check size_hint is not changed after exhausted
        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_iter_raw() {

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        let mut it = nvec.iter_raw();
        assert_eq!(it.size_hint(), (3, Some(3)));

        assert_eq!(it.next(), Some((true, &1)));
        assert_eq!(it.size_hint(), (2, Some(2)));

        assert_eq!(it.next(), Some((false, &2)));
        assert_eq!(it.size_hint(), (1, Some(1)));

        assert_eq!(it.next(), Some((false, &3)));
        assert_eq!(it.size_hint(), (0, Some(0)));

        // check size_hint is not changed after exhausted
        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_iter_not_null() {

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        let mut it = nvec.iter_not_null();
        assert_eq!(it.size_hint(), (3, Some(3)));

        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.size_hint(), (1, Some(1)));

        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.size_hint(), (0, Some(0)));

        // check size_hint is not changed after exhausted
        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));
    }

    fn test_iter_not_null_long() {
        let values: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nvec = NullVec::with_mask(values,
                                      Some(vec![true, false, false, true, true, true, false,
                                                true, true]));
        let mut it = nvec.iter_not_null();
        assert_eq!(it.size_hint(), (9, Some(9)));

        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.size_hint(), (7, Some(7)));

        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.size_hint(), (6, Some(6)));

        assert_eq!(it.next(), Some(&7));
        assert_eq!(it.size_hint(), (2, Some(2)));

        // check size_hint is not changed after exhausted
        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));

        assert_eq!(it.next(), None);
        assert_eq!(it.size_hint(), (0, Some(0)));
    }
}
