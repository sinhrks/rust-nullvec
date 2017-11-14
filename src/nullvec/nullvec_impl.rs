
use algos::indexing::Indexing;

use super::NullVec;
use nullable::Nullable;
use traits::{NullStorable, Slicer, Stringify, Append};

impl<T: Clone + NullStorable> NullVec<T> {
    pub fn has_null(&self) -> bool {
        match self.mask {
            Some(_) => true,
            None => false,
        }
    }

    /// Returns `Vec<bool>` whether the collesponding value is `Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nullvec::prelude::*;
    /// let v = NullVec::with_mask(vec![1, 2, 3], Some(vec![false, true, false]));
    /// assert_eq!(v.is_null(), vec![false, true, false]);
    /// ```
    pub fn is_null(&self) -> Vec<bool> {
        match self.mask {
            Some(ref mask) => mask.clone(),
            None => vec![false; self.len()],
        }
    }

    /// Returns `Vec<bool>` whether the collesponding value is not `Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nullvec::prelude::*;
    /// let v = NullVec::with_mask(vec![1, 2, 3], Some(vec![false, true, false]));
    /// assert_eq!(v.not_null(), vec![true, false, true]);
    /// ```
    pub fn not_null(&self) -> Vec<bool> {
        match self.mask {
            Some(ref mask) => mask.iter().map(|x| !x).collect::<Vec<bool>>(),
            None => vec![true; self.len()],
        }
    }

    /// Returns `Vec<T>` of values which is not `Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nullvec::prelude::*;
    /// let v = NullVec::with_mask(vec![1, 2, 3], Some(vec![false, true, false]));
    /// assert_eq!(v.not_null_values(), vec![1, 3]);
    /// ```
    pub fn not_null_values(&self) -> Vec<T> {
        match self.mask {
            Some(ref mask) => {
                mask.iter()
                    .zip(self.data.iter())
                    .filter(|&(&m, _)| m == false)
                    .map(|(_, v)| v)
                    .cloned()
                    .collect::<Vec<T>>()
            }
            None => self.data.clone(),
        }
    }
}


impl<T: Clone + NullStorable> NullVec<T> {
    /// Returns `NullVec<T>` which has the same length as the caller
    /// whose values are all `Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use nullvec::prelude::*;
    /// let v = NullVec::new(vec![1, 2, 3]);
    /// assert_eq!(v.as_null().is_null(), vec![true, true, true]);
    /// ```
    pub fn as_null(&self) -> Self {
        NullVec {
            data: self.data.clone(),
            mask: Some(vec![true; self.len()]),
        }
    }

    /// Returns `NullVec<T>` of not `Null` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use nullvec::prelude::*;
    /// let v = NullVec::with_mask(vec![1, 2, 3], Some(vec![false, true, false]));
    /// assert_eq!(v.drop_null(), NullVec::new(vec![1, 3]));
    /// ```
    pub fn drop_null(&self) -> Self {
        // ToDo: merge with not_null_values?
        match &self.mask {
            &Some(ref mask) => {
                let new_values: Vec<T> = mask.iter()
                    .zip(self.data.iter())
                    .filter(|&(&m, _)| !m)
                    .map(|(_, v)| v.clone())
                    .collect();

                NullVec {
                    data: new_values,
                    mask: None,
                }
            }
            &None => self.clone(),
        }
    }

    /// Returns `NullVec<T>` filling `Null` with specified value.
    ///
    /// # Examples
    ///
    /// ```
    /// use nullvec::prelude::*;
    /// let v = NullVec::with_mask(vec![1, 2, 3], Some(vec![false, true, false]));
    /// assert_eq!(v.fill_null(5), NullVec::new(vec![1, 5, 3]));
    /// ```
    pub fn fill_null(&self, value: T) -> Self {
        match &self.mask {
            &Some(ref mask) => {
                let new_values: Vec<T> = mask.iter()
                    .zip(self.data.iter())
                    .map(|(&m, v)| if m == true { value.clone() } else { v.clone() })
                    .collect();

                NullVec {
                    data: new_values,
                    mask: None,
                }
            }
            &None => self.clone(),
        }
    }
}

impl<T: Clone + NullStorable> Slicer for NullVec<T> {
    type Scalar = Nullable<T>;

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iloc(&self, location: &usize) -> Self::Scalar {
        assert!(*location < self.len(), "Index out of bounds");
        unsafe { self.iloc_unchecked(location) }
    }

    unsafe fn iloc_unchecked(&self, location: &usize) -> Self::Scalar {
        match self.mask {
            Some(ref mask) => {
                if *mask.get_unchecked(*location) == true {
                    Nullable::Null
                } else {
                    Nullable::Value(self.data.get_unchecked(*location).clone())
                }
            }
            None => Nullable::Value(self.data.get_unchecked(*location).clone()),
        }
    }

    fn ilocs(&self, locations: &[usize]) -> Self {
        Indexing::assert_index_boundary(&self.data, locations);
        unsafe { self.ilocs_unchecked(locations) }
    }

    unsafe fn ilocs_unchecked(&self, locations: &[usize]) -> Self {
        let new_data = Indexing::reindex_unchecked(&self.data, locations);
        let new_mask = match self.mask {
            Some(ref mask) => Some(Indexing::reindex_unchecked(mask, locations)),
            None => None,
        };
        NullVec {
            data: new_data,
            mask: new_mask,
        }
    }

    fn ilocs_forced(&self, locations: &[usize]) -> Self {
        let mut new_data: Vec<T> = Vec::with_capacity(locations.len());
        let mut new_mask: Vec<bool> = Vec::with_capacity(locations.len());
        // check whether result have null
        let mut has_null = false;

        match self.mask {
            Some(ref mask) => {
                for loc in locations.iter() {
                    if *loc >= self.len() {

                        new_data.push(T::default());
                        new_mask.push(true);
                        has_null = true;
                    } else {
                        new_data.push((&self.data[*loc]).clone());
                        if mask[*loc] == true {
                            new_mask.push(true);
                            has_null = true;
                        } else {
                            new_mask.push(false);
                        }
                    }
                }
            }
            None => {
                for loc in locations.iter() {
                    if *loc >= self.len() {
                        new_data.push(T::default());
                        new_mask.push(true);
                        has_null = true;
                    } else {
                        new_data.push((&self.data[*loc]).clone());
                        new_mask.push(false);
                    }
                }
            }
        }
        if has_null == true {
            NullVec {
                data: new_data,
                mask: Some(new_mask),
            }
        } else {
            NullVec {
                data: new_data,
                mask: None,
            }
        }
    }

    fn blocs(&self, flags: &[bool]) -> Self {
        let new_data = Indexing::blocs(&self.data, flags);
        // ToDo: can use unsafe
        let new_mask = match self.mask {
            Some(ref mask) => Some(Indexing::blocs(mask, flags)),
            None => None,
        };
        NullVec {
            data: new_data,
            mask: new_mask,
        }
    }
}

impl<T: Clone + NullStorable> Append for NullVec<T> {
    fn append(&self, other: &NullVec<T>) -> Self {
        let new_data: Vec<T> = self.data
            .iter()
            .cloned()
            .chain(other.data.iter().cloned())
            .collect();
        let new_mask = match (&self.mask, &other.mask) {
            (&None, &None) => None,
            (&Some(ref lmask), &None) => {
                let new_mask: Vec<bool> = lmask
                    .iter()
                    .cloned()
                    .chain(other.is_null().into_iter())
                    .collect();
                Some(new_mask)
            }
            (&None, &Some(ref rmask)) => {
                let new_mask: Vec<bool> = other
                    .is_null()
                    .into_iter()
                    .chain(rmask.iter().cloned())
                    .collect();
                Some(new_mask)
            }
            (&Some(ref lmask), &Some(ref rmask)) => {
                let new_mask: Vec<bool> =
                    lmask.iter().cloned().chain(rmask.iter().cloned()).collect();
                Some(new_mask)
            }
        };
        NullVec::with_mask(new_data, new_mask)
    }
}

impl<T> Stringify for NullVec<T>
where
    T: NullStorable + Clone + ToString,
{
    fn into_string_vec(&self) -> Vec<String> {
        match self.mask {
            Some(ref mask) => {
                self.data
                    .iter()
                    .zip(mask.iter())
                    .map(|(v, &m)| if m == true {
                        "Null".to_string()
                    } else {
                        v.clone().to_string()
                    })
                    .collect()
            }
            None => self.data.iter().map(|x| x.to_string()).collect(),
        }
    }
}


#[cfg(test)]
mod tests {

    use std::f64;

    use nullable::Nullable;
    use nullvec::NullVec;
    use traits::{Slicer, Stringify};

    #[test]
    fn test_int_isnull() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.has_null(), false);
        let res = nvec.is_null();
        assert_eq!(res, vec![false, false, false]);

        let res = nvec.not_null();
        assert_eq!(res, vec![true, true, true]);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        assert_eq!(nvec.has_null(), true);
        let res = nvec.is_null();
        assert_eq!(res, vec![false, false, true]);

        let res = nvec.not_null();
        assert_eq!(res, vec![true, true, false]);
    }

    #[test]
    fn test_float_isnull() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.has_null(), true);
        let res = nvec.is_null();
        assert_eq!(res, vec![false, true, false]);

        let res = nvec.not_null();
        assert_eq!(res, vec![true, false, true]);
    }

    #[test]
    fn test_float_not_null_values() {
        let values: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.not_null_values(), vec![1.1, 1.2, 1.3]);

        let values: Vec<f64> = vec![f64::NAN, 1.2, 1.3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.not_null_values(), vec![1.2, 1.3]);
    }

    #[test]
    fn test_as_null() {
        let values: Vec<i64> = vec![1, 2, 3];
        let nvec = NullVec::new(values);

        let res = nvec.as_null();
        assert_eq!(res.data, vec![1, 2, 3]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }

    #[test]
    fn test_int_drop_null() {
        let values: Vec<usize> = vec![1, 0, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, true, false]));

        let res = nvec.drop_null();
        assert_eq!(res.data, vec![1, 3]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_fill_null() {
        let values: Vec<usize> = vec![1, 0, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, true, false]));

        let res = nvec.fill_null(10);
        assert_eq!(res.data, vec![1, 10, 3]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float_drop_null() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);

        let res = nvec.drop_null();
        assert_eq!(res.data, vec![1.1, 1.3]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float_fill_null() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);

        let res = nvec.fill_null(10.0);
        assert_eq!(res.data, vec![1.1, 10.0, 1.3]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_iloc() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.iloc(&1), Nullable::Value(2));

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        assert_eq!(nvec.iloc(&1), Nullable::Value(2));
        assert_eq!(nvec.iloc(&2), Nullable::Null);
    }

    #[test]
    #[should_panic]
    fn test_iloc_out_of_bounds1() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        nvec.iloc(&3);
    }

    #[test]
    #[should_panic]
    fn test_iloc_out_of_bounds2() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        nvec.iloc(&3);
    }

    #[test]
    fn test_iloc_unchecked() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(unsafe { nvec.iloc_unchecked(&1) }, Nullable::Value(2));

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        assert_eq!(unsafe { nvec.iloc_unchecked(&1) }, Nullable::Value(2));
        assert_eq!(unsafe { nvec.iloc_unchecked(&2) }, Nullable::Null);
    }

    #[test]
    fn test_ilocs() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec.ilocs(&vec![2, 1]);
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, None);

        // select with slice
        let res = nvec.ilocs(&[2, 1]);
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, None);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        let res = nvec.ilocs(&vec![2, 1]);
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, Some(vec![true, false]));

        // select with slice
        let res = nvec.ilocs(&[2, 1]);
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, Some(vec![true, false]));
    }

    #[test]
    #[should_panic]
    fn test_ilocs_out_of_bounds1() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        nvec.ilocs(&vec![1, 3]);
    }

    #[test]
    #[should_panic]
    fn test_ilocs_out_of_bounds2() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        nvec.ilocs(&vec![1, 3]);
    }

    #[test]
    fn test_ilocs_unchecked() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = unsafe { nvec.ilocs_unchecked(&vec![2, 1]) };
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, None);

        // select with slice
        let res = unsafe { nvec.ilocs_unchecked(&[2, 1]) };
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, None);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        let res = unsafe { nvec.ilocs_unchecked(&vec![2, 1]) };
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, Some(vec![true, false]));

        // select with slice
        let res = unsafe { nvec.ilocs_unchecked(&vec![2, 1]) };
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, Some(vec![true, false]));
    }

    #[test]
    fn test_ilocs_forced() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec.ilocs_forced(&vec![2, 1]);
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, None);

        let res = nvec.ilocs_forced(&vec![5, 1]);
        assert_eq!(res.data, vec![0, 2]);
        assert_eq!(res.mask, Some(vec![true, false]));

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        let res = nvec.ilocs_forced(&vec![2, 1]);
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, Some(vec![true, false]));

        let res = nvec.ilocs_forced(&vec![0, 1]);
        assert_eq!(res.data, vec![1, 2]);
        assert_eq!(res.mask, None);

        let res = nvec.ilocs_forced(&vec![10, 10]);
        assert_eq!(res.data, vec![0, 0]);
        assert_eq!(res.mask, Some(vec![true, true]));

        // select with slice
        let res = nvec.ilocs_forced(&vec![2, 1]);
        assert_eq!(res.data, vec![3, 2]);
        assert_eq!(res.mask, Some(vec![true, false]));
    }

    #[test]
    fn test_ilocs_forced_empty() {
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        let res = nvec.ilocs_forced(&vec![2, 1]);
        assert_eq!(res.data, vec![0, 0]);
        assert_eq!(res.mask, Some(vec![true, true]));
    }

    #[test]
    fn test_into_string_vec() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let exp = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        assert_eq!(nvec.into_string_vec(), exp);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        let exp = vec!["1".to_string(), "2".to_string(), "Null".to_string()];
        assert_eq!(nvec.into_string_vec(), exp);

        let values: Vec<String> = vec!["a".to_string(), "bb".to_string(), "".to_string()];
        let nvec = NullVec::with_mask(values, Some(vec![false, false, true]));
        let exp = vec!["a".to_string(), "bb".to_string(), "Null".to_string()];
        assert_eq!(nvec.into_string_vec(), exp);
    }
}
