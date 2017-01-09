// use std::collections::BitVec;

use num::Float;

mod nullvec_impl;

// aggregation
mod nullvec_aggregation;

// convert
mod nullvec_convert;

// iterator
mod nullvec_iter;

// broadcast op
mod nullvec_primitive_ops;
mod nullvec_nullable_ops;

// elemwise op
mod nullvec_vec_ops;
mod nullvec_nullvec_ops;

use algos::vec_ops::Elemwise;
use traits::NullStorable;

#[derive(Clone, Debug, PartialEq)]
pub struct NullVec<T: NullStorable> {
    data: Vec<T>,
    // ToDo: use BitVec
    mask: Option<Vec<bool>>,
}

fn maybe_null<T: NullStorable>(values: Vec<T>) -> (Vec<T>, Option<Vec<bool>>) {
    if T::has_primitive_null() {
        let mut not_null: Vec<T> = Vec::with_capacity(values.len());
        let mut mask: Vec<bool> = Vec::with_capacity(values.len());
        let mut has_null = false;

        for v in values.into_iter() {
            if v.is_null() {
                not_null.push(T::default());
                mask.push(true);
                has_null = true;
            } else {
                not_null.push(v);
                mask.push(false);
            }
        }
        if has_null == true {
            (not_null, Some(mask))
        } else {
            (not_null, None)
        }
    } else {
        (values, None)
    }

}

impl<T: NullStorable> NullVec<T> {
    pub fn new(values: Vec<T>) -> Self {
        let (not_null, mask) = maybe_null(values);

        NullVec {
            data: not_null,
            mask: mask,
        }
    }

    fn with_mask(values: Vec<T>, mask: Option<Vec<bool>>) -> Self {
        let (not_null, null_mask) = maybe_null(values);
        let new_mask = match (null_mask, mask) {
            (Some(lmask), Some(rmask)) => Some(Elemwise::elemwise_oo(lmask, rmask, |x, y| x | y)),
            (Some(lmask), None) => Some(lmask),
            (None, Some(rmask)) => Some(rmask),
            (None, None) => None,
        };

        NullVec {
            data: not_null,
            mask: new_mask,
        }
    }
}


#[cfg(test)]
mod tests {

    use std::f64;

    use super::{NullVec, maybe_null};

    #[test]
    fn test_int() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.data, vec![1, 2, 3]);
        assert_eq!(nvec.mask, None);
    }

    #[test]
    fn test_float() {
        let values: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.data, vec![1.1, 1.2, 1.3]);
        assert_eq!(nvec.mask, None);
    }

    #[test]
    fn test_float_nan() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.data, vec![1.1, 0., 1.3]);
        assert_eq!(nvec.mask, Some(vec![false, true, false]));
    }

    #[test]
    fn test_int_with_mask() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.data, vec![1, 2, 3]);
        assert_eq!(nvec.mask, Some(vec![true, false, false]));
    }

    #[test]
    fn test_float_with_mask() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.data, vec![1.1, 0., 1.3]);
        assert_eq!(nvec.mask, Some(vec![true, true, false]));

        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::with_mask(values, None);
        assert_eq!(nvec.data, vec![1.1, 0., 1.3]);
        assert_eq!(nvec.mask, Some(vec![false, true, false]));
    }

    #[test]
    fn test_maybe_null() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let (not_null, mask) = maybe_null(values);
        assert_eq!(not_null, vec![1.1, 0., 1.3]);
        assert_eq!(mask, Some(vec![false, true, false]));

        let values: Vec<f64> = vec![1.1, 1.2, 1.3];
        let (not_null, mask) = maybe_null(values);
        assert_eq!(not_null, vec![1.1, 1.2, 1.3]);
        assert_eq!(mask, None);
    }
}
