// use std::collections::BitVec;

use num::Float;

#[macro_use]
use macros;

// temp, remove
macro_rules! dispatch {
    ($m:ident) => {
        $m!(i64);
        $m!(i32);
        $m!(i16);
        $m!(i8);
        $m!(isize);
        $m!(u64);
        $m!(u32);
        $m!(u16);
        $m!(u8);
        $m!(usize);
        $m!(bool);
        $m!(String);
    }
}

use traits::TypeDispatch;

pub struct NullVec<T> {
    data: Vec<T>,
    // ToDo: use BitVec
    mask: Option<Vec<bool>>
}


macro_rules! impl_new_never_nullable {
    ($t:ident) => {
        impl TypeDispatch<$t> for NullVec<$t> {
            fn new(values: Vec<$t>) -> NullVec<$t> {
                NullVec {
                    data: values,
                    mask: None
                }
            }
        }
    }
}
dispatch!(impl_new_never_nullable);


macro_rules! impl_new_nullable {
    ($t:ident) => {
        impl TypeDispatch<$t> for NullVec<$t> {
            fn new(values: Vec<$t>) -> NullVec<$t> {

                let mut not_null: Vec<$t> = Vec::with_capacity(values.len());
                let mut mask: Vec<bool> = Vec::with_capacity(values.len());
                let mut has_null = false;

                for v in values.into_iter() {
                    if v.is_nan() {
                        not_null.push(0.);
                        mask.push(true);
                        has_null = true;
                    } else {
                        not_null.push(v);
                        mask.push(false);
                    }
                }
                if has_null == true {
                    NullVec {
                        data: not_null,
                        mask: Some(mask)
                    }
                } else {
                    NullVec {
                        data: not_null,
                        mask: None
                    }
                }
            }
        }
    }
}
impl_new_nullable!(f64);
impl_new_nullable!(f32);


#[cfg(test)]
mod tests {

    use std::f64;

    use super::NullVec;
    use traits::TypeDispatch;

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
}
