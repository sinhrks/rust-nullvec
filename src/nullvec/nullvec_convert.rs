use super::NullVec;
use nullable::Nullable;
use traits::NullStorable;

impl<T: NullStorable> From<Vec<T>> for NullVec<T> {
    fn from(values: Vec<T>) -> Self {
        NullVec::new(values)
    }
}

impl<T: NullStorable> From<Vec<Nullable<T>>> for NullVec<T> {
    fn from(values: Vec<Nullable<T>>) -> Self {
        let mut new_values: Vec<T> = Vec::with_capacity(values.len());
        let mut new_mask: Vec<bool> = Vec::with_capacity(values.len());
        let mut has_null: bool = false;
        for v in values.into_iter() {
            match v {
                Nullable::Value(v) => {
                    new_values.push(v);
                    new_mask.push(false);
                }
                Nullable::Null => {
                    new_values.push(T::default());
                    new_mask.push(true);
                    has_null = true;
                }
            }
        }
        if has_null {
            NullVec {
                data: new_values,
                mask: Some(new_mask),
            }
        } else {
            NullVec {
                data: new_values,
                mask: None,
            }
        }
    }
}

macro_rules! impl_from {
    ($t:ident) => {
        impl From<NullVec<$t>> for Vec<$t> {
            fn from(values: NullVec<$t>) -> Vec<$t> {
                // ToDo: check has_primitive_null and return type null
                match values.mask {
                    None => values.data,
                    // ToDo: must be TryFrom
                    _ => panic!("Unable to convert NaN to int")
                }
            }
        }

        impl From<NullVec<$t>> for Vec<Nullable<$t>> {
            fn from(values: NullVec<$t>) -> Vec<Nullable<$t>> {
                // ToDo: check has_primitive_null and return type null
                match values.mask {
                    None => values.data
                                  .into_iter()
                                  .map(|x| Nullable::Value(x))
                                  .collect(),
                    Some(mask) => values.data
                                  .into_iter()
                                  .zip(mask.into_iter())
                                  .map(|(x, m)| if m == true {
                                                    Nullable::Null
                                                } else {
                                                    Nullable::Value(x)
                                                })
                                  .collect(),

                }
            }
        }
    }
}
macro_dispatch!(impl_from,
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
                f64,
                f32,
                bool,
                String);

#[cfg(test)]
mod tests {

    use std::f64;
    use nullable::Nullable;
    use nullvec::NullVec;

    #[test]
    fn test_float_conv_from_vec() {
        let nv = NullVec::<f64>::from(vec![1., 2.]);
        assert_eq!(nv.data, vec![1., 2.]);
        assert_eq!(nv.mask, None);

        let res = Vec::<f64>::from(nv);
        assert_eq!(res, vec![1., 2.]);

        let nv = NullVec::<f64>::from(vec![1., f64::NAN]);
        assert_eq!(nv.data, vec![1., 0.]);
        assert_eq!(nv.mask, Some(vec![false, true]));
    }

    #[test]
    #[should_panic]
    fn test_float_conv_from_vec_contains_null() {

        let nv = NullVec::<f64>::from(vec![1., f64::NAN]);
        assert_eq!(nv.data, vec![1., 0.]);
        assert_eq!(nv.mask, Some(vec![false, true]));

        // ToDo: this must success
        Vec::<f64>::from(nv);
    }

    #[test]
    fn test_float_conv_from_nullablevec() {
        let nv = NullVec::<f64>::from(vec![Nullable::Value(1.), Nullable::Value(2.)]);
        assert_eq!(nv.data, vec![1., 2.]);
        assert_eq!(nv.mask, None);

        let res = Vec::<Nullable<f64>>::from(nv);
        assert_eq!(res, vec![Nullable::Value(1.), Nullable::Value(2.)]);

        let nv = NullVec::<f64>::from(vec![Nullable::Value(1.), Nullable::Null]);
        assert_eq!(nv.data, vec![1., 0.]);
        assert_eq!(nv.mask, Some(vec![false, true]));

        let res = Vec::<Nullable<f64>>::from(nv);
        assert_eq!(res, vec![Nullable::Value(1.), Nullable::Null]);
    }
}
