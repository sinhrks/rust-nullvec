// use std::collections::BitVec;

use num::Float;

mod nullvec_impl;

// aggregation
mod nullvec_aggregation;

// iterator
mod nullvec_iter;

// broadcast op
mod nullvec_primitive_ops;
mod nullvec_nullable_ops;

// elemwise op
mod nullvec_vec_ops;
mod nullvec_nullvec_ops;

use algos::vec_ops::Elemwise;
use traits::VecBase;

#[derive(Clone, Debug, PartialEq)]
pub struct NullVec<T: Clone> {
    data: Vec<T>,
    // ToDo: use BitVec
    mask: Option<Vec<bool>>,
}

macro_rules! impl_new_never_nullable {
    ($t:ident) => {
        impl VecBase<$t> for NullVec<$t> {
            fn new(values: Vec<$t>) -> Self {
                NullVec {
                    data: values,
                    mask: None
                }
            }

            fn with_mask(values: Vec<$t>, mask: Option<Vec<bool>>) -> Self {
                NullVec {
                    data: values,
                    mask: mask
                }
            }
        }
    }
}
macro_dispatch!(impl_new_never_nullable,
                i64,
                i32,
                i16,
                i8,
                isize,
                u64,
                u32,
                u16,
                u8,
                usize,
                bool,
                String);

fn maybe_null<T: Float>(values: Vec<T>) -> (Vec<T>, Option<Vec<bool>>) {

    let mut not_null: Vec<T> = Vec::with_capacity(values.len());
    let mut mask: Vec<bool> = Vec::with_capacity(values.len());
    let mut has_null = false;

    for v in values.into_iter() {
        if v.is_nan() {
            not_null.push(T::zero());
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
}

macro_rules! impl_new_nullable {
    ($t:ident) => {
        impl VecBase<$t> for NullVec<$t> {
            fn new(values: Vec<$t>) -> Self {
                let (not_null, mask) = maybe_null(values);

                NullVec {
                    data: not_null,
                    mask: mask
                }
            }

            fn with_mask(values: Vec<$t>, mask: Option<Vec<bool>>) -> Self {
                let (not_null, null_mask) = maybe_null(values);
                let new_mask = match (null_mask, mask) {
                    (Some(lmask), Some(rmask)) => Some(Elemwise::elemwise_oo(lmask,
                                                                             rmask,
                                                                             |x, y| x | y)),
                    (Some(lmask), None) => Some(lmask),
                    (None, Some(rmask)) => Some(rmask),
                    (None, None) => None
                };

                NullVec {
                    data: not_null,
                    mask: new_mask
                }
            }

        }
    }
}
macro_dispatch!(impl_new_nullable, f64, f32);

#[cfg(test)]
mod tests {

    use std::f64;

    use super::{NullVec, maybe_null};
    use traits::VecBase;

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
