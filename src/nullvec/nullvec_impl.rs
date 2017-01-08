
use num::Float;

use super::NullVec;

impl<T> NullVec<T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn has_null(&self) -> bool {
        match self.mask {
            Some(_) => true,
            None => false,
        }
    }

    /// Returns bool Vec whether the collesponding value is Null or not
    pub fn is_null(&self) -> Vec<bool> {
        match self.mask {
            Some(ref mask) => mask.clone(),
            None => vec![false; self.len()],
        }
    }

    pub fn not_null(&self) -> Vec<bool> {
        match self.mask {
            Some(ref mask) => mask.iter().map(|x| !x).collect::<Vec<bool>>(),
            None => vec![true; self.len()],
        }
    }
}

impl<T: Clone> Clone for NullVec<T> {
    fn clone(&self) -> Self {
        NullVec {
            data: self.data.clone(),
            mask: self.mask.clone(),
        }
    }
}

impl<T: Clone> NullVec<T> {
    /// Returns NullVec which has the same length as the caller
    /// whose values are all Null
    pub fn as_null(&self) -> Self {
        NullVec {
            data: self.data.clone(),
            mask: Some(vec![true; self.len()]),
        }
    }

    pub fn drop_null(&self) -> Self {
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


#[cfg(test)]
mod tests {

    use std::f64;

    use nullvec::NullVec;
    use traits::TypeDispatchVec;

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
}
