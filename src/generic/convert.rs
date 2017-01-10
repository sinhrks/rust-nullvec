use super::{Array, Scalar};
use nullable::Nullable;
use nullvec::NullVec;

// ToDo: check all impls are enough efficient

////////////////////////////////////////////////////////////////////////////////
// Vec to Array
////////////////////////////////////////////////////////////////////////////////

macro_rules! add_array_conversion {
    ($t:ident, $klass:ident) => {

        impl From<Vec<$t>> for Array {
            fn from(values: Vec<$t>) -> Self {
                Array::$klass(NullVec::new(values))
            }
        }

        impl From<Array> for Vec<$t> {
            fn from(values: Array) -> Self {
                match values {
                    Array::$klass(vals) => vals.into(),
                    _ => panic!("Unable to convert to Vec")
                }
            }
        }

        // Vec<Nullable<T>>
        impl From<Array> for Vec<Nullable<$t>> {
            fn from(values: Array) -> Self {
                match values {
                    Array::$klass(vals) => vals.into(),
                    _ => panic!("Unable to convert to Vec")
                }
            }
        }

        // NullVec<T>
        impl From<NullVec<$t>> for Array {
            fn from(values: NullVec<$t>) -> Self {
                Array::$klass(values)
            }
        }

        impl From<Array> for NullVec<$t> {
            fn from(values: Array) -> Self {
                match values {
                    Array::$klass(vals) => vals,
                    _ => panic!("Unable to convert to Vec")
                }
            }
        }
    }
}
add_array_conversion!(i64, Int64Array);
add_array_conversion!(i32, Int32Array);
add_array_conversion!(i16, Int16Array);
add_array_conversion!(i8, Int8Array);
add_array_conversion!(isize, IsizeArray);
add_array_conversion!(u64, UInt64Array);
add_array_conversion!(u32, UInt32Array);
add_array_conversion!(u16, UInt16Array);
add_array_conversion!(u8, UInt8Array);
add_array_conversion!(usize, UsizeArray);
add_array_conversion!(f64, Float64Array);
add_array_conversion!(f32, Float32Array);
add_array_conversion!(bool, BoolArray);
add_array_conversion!(String, StringArray);

// &str handling
//
// impl<'a> From<Vec<&'a str>> for Array {
// fn from(values: Vec<&'a str>) -> Self {
// let new_values: Vec<String> = values.iter().map(|&x| String::from(x)).collect();
// Array::StringArray(new_values)
// }
// }
//

////////////////////////////////////////////////////////////////////////////////
// Scalar Vec to Array
////////////////////////////////////////////////////////////////////////////////

impl From<Vec<Scalar>> for Array {
    fn from(values: Vec<Scalar>) -> Self {
        assert!(values.len() > 0, "Unable to infer dtype");

        // ToDo: use better logic to infer dtype (take first not null?)
        // ToDo: Directly create Nullable internal data
        match &values[0] {
            &Scalar::i64(_) => {
                values.iter().map(|ref x| x.as_i64()).collect::<NullVec<i64>>().into()
            }
            &Scalar::i32(_) => {
                values.iter().map(|ref x| x.as_i32()).collect::<NullVec<i32>>().into()
            }
            &Scalar::i16(_) => {
                values.iter().map(|ref x| x.as_i16()).collect::<NullVec<i16>>().into()
            }
            &Scalar::i8(_) => values.iter().map(|ref x| x.as_i8()).collect::<NullVec<i8>>().into(),
            &Scalar::isize(_) => {
                values.iter().map(|ref x| x.as_isize()).collect::<NullVec<isize>>().into()
            }
            &Scalar::u64(_) => {
                values.iter().map(|ref x| x.as_u64()).collect::<NullVec<u64>>().into()
            }
            &Scalar::u32(_) => {
                values.iter().map(|ref x| x.as_u32()).collect::<NullVec<u32>>().into()
            }
            &Scalar::u16(_) => {
                values.iter().map(|ref x| x.as_u16()).collect::<NullVec<u16>>().into()
            }
            &Scalar::u8(_) => values.iter().map(|ref x| x.as_u8()).collect::<NullVec<u8>>().into(),
            &Scalar::usize(_) => {
                values.iter().map(|ref x| x.as_usize()).collect::<NullVec<usize>>().into()
            }
            &Scalar::f64(_) => {
                values.iter().map(|ref x| x.as_f64()).collect::<NullVec<f64>>().into()
            }
            &Scalar::f32(_) => {
                values.iter().map(|ref x| x.as_f32()).collect::<NullVec<f32>>().into()
            }
            &Scalar::bool(_) => {
                values.iter().map(|ref x| x.as_bool()).collect::<NullVec<bool>>().into()
            }
            &Scalar::String(_) => {
                values.iter().map(|ref x| x.as_str()).collect::<NullVec<String>>().into()
            }
            // ToDo: Fix me
            &Scalar::Null => panic!("unable to infer dtype"),
        }
    }
}

impl From<Array> for Vec<Scalar> {
    fn from(values: Array) -> Self {
        match values {
            // ToDo: Directly create Scalar from NullVec Internal (do not call iter)
            Array::Int64Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::i64(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::Int32Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::i32(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::Int16Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::i16(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::Int8Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::i8(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::IsizeArray(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::isize(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::UInt64Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::u64(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::UInt32Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::u32(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::UInt16Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::u16(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::UInt8Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::u8(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::UsizeArray(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::usize(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::Float64Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::f64(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::Float32Array(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::f32(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::BoolArray(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::bool(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
            Array::StringArray(vals) => {
                vals.into_iter()
                    .map(|x| match x {
                        Nullable::Value(val) => Scalar::String(val),
                        Nullable::Null => Scalar::Null,
                    })
                    .collect()
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Scalar to primitives
////////////////////////////////////////////////////////////////////////////////

macro_rules! add_scalar_conversion {
    ($t:ident) => {

        impl From<$t> for Scalar {
            fn from(value: $t) -> Self {
                Scalar::$t(value)
            }
        }

        impl From<Nullable<$t>> for Scalar {
            fn from(value: Nullable<$t>) -> Self {
                match value {
                    Nullable::Null => Scalar::Null,
                    Nullable::Value(val) => Scalar::$t(val),
                }
            }
        }

        impl From<Scalar> for $t {
            fn from(value: Scalar) -> Self {
                match value {
                    Scalar::$t(val) => val,
                    _ => panic!("Unable to convert to primitive")
                }
            }
        }

        impl From<Scalar> for Nullable<$t> {
            fn from(value: Scalar) -> Self {
                match value {
                    Scalar::Null => Nullable::Null,
                    Scalar::$t(val) => Nullable::Value(val),
                    _ => panic!("Unable to convert to primitive")
                }
            }
        }
    }
}
add_scalar_conversion!(i64);
add_scalar_conversion!(i32);
add_scalar_conversion!(i16);
add_scalar_conversion!(i8);
add_scalar_conversion!(isize);
add_scalar_conversion!(u64);
add_scalar_conversion!(u32);
add_scalar_conversion!(u16);
add_scalar_conversion!(u8);
add_scalar_conversion!(usize);
add_scalar_conversion!(f64);
add_scalar_conversion!(f32);
add_scalar_conversion!(bool);
add_scalar_conversion!(String);

// &str handling
//
// impl<'a> From<&'a str> for Scalar {
// fn from(value: &'a str) -> Self {
// Scalar::String(value.to_string())
// }
// }
//


#[cfg(test)]
mod tests {

    use super::super::{Array, Scalar};
    use nullvec::NullVec;

    #[test]
    #[should_panic]
    fn test_empty_scalar_to_array() {
        let vals: Vec<Scalar> = vec![];
        let _: Array = vals.into();
    }

    #[test]
    fn test_i64_vec_to_array() {
        let exp: Array = Array::Int64Array(NullVec::new(vec![1, 2]));

        // Into
        let vals: Vec<i64> = vec![1, 2];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<i64> = vec![1, 2];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_i64_array_to_vec() {
        let exp: Vec<i64> = vec![1, 2];
        let exps: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];

        // Into
        let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
        let res: Vec<i64> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
        let res: Vec<i64> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_usize_vec_to_array() {
        let exp: Array = Array::UsizeArray(NullVec::new(vec![1, 2]));

        // Into
        let vals: Vec<usize> = vec![1, 2];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::usize(1), Scalar::usize(2)];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<usize> = vec![1, 2];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::usize(1), Scalar::usize(2)];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_usize_array_to_vec() {
        let exp: Vec<usize> = vec![1, 2];
        let exps: Vec<Scalar> = vec![Scalar::usize(1), Scalar::usize(2)];

        // Into
        let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
        let res: Vec<usize> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
        let res: Vec<usize> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_f64_vec_to_array() {
        let exp: Array = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));

        // Into
        let vals: Vec<f64> = vec![1.1, 2.2];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<f64> = vec![1.1, 2.2];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_f64_array_to_vec() {
        let exp: Vec<f64> = vec![1.1, 2.2];
        let exps: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];

        // Into
        let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
        let res: Vec<f64> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
        let res: Vec<f64> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_bool_vec_to_array() {
        let exp: Array = Array::BoolArray(NullVec::new(vec![true, false]));

        // Into
        let vals: Vec<bool> = vec![true, false];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<bool> = vec![true, false];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_bool_array_to_vec() {
        let exp: Vec<bool> = vec![true, false];
        let exps: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];

        // Into
        let vals = Array::BoolArray(NullVec::new(vec![true, false]));
        let res: Vec<bool> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::BoolArray(NullVec::new(vec![true, false]));
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::BoolArray(NullVec::new(vec![true, false]));
        let res: Vec<bool> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::BoolArray(NullVec::new(vec![true, false]));
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_str_vec_to_array() {
        let exp: Array = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));

        // Into
        let vals: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // let vals: Vec<&str> = vec!["a", "b"];
        // let res: Array = vals.into();
        // assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::String("a".to_string()),
                                     Scalar::String("b".to_string())];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let res = Array::from(vals);
        assert_eq!(res, exp);


        // let vals: Vec<&str> = vec!["a", "b"];
        // let res = Array::from(vals);
        // assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::String("a".to_string()),
                                     Scalar::String("b".to_string())];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_str_array_to_vec() {
        let exp: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let exps: Vec<Scalar> = vec![Scalar::String("a".to_string()),
                                     Scalar::String("b".to_string())];

        // Into
        let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
        let res: Vec<String> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
        let res: Vec<String> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_i64_primitives_to_scalar() {
        let exp = Scalar::i64(1);

        let res: Scalar = 1i64.into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from(1i64);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_i64_scalar_to_primitives() {
        let res: i64 = Scalar::i64(1).into();
        assert_eq!(res, 1);

        let res: i64 = i64::from(Scalar::i64(1));
        assert_eq!(res, 1);
    }

    #[test]
    fn test_f64_primitives_to_scalar() {
        let exp = Scalar::f64(1.1);

        let res: Scalar = (1.1).into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from(1.1);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_f64_scalar_to_primitives() {
        let res: f64 = Scalar::f64(1.1).into();
        assert_eq!(res, 1.1);

        let res: f64 = f64::from(Scalar::f64(1.1));
        assert_eq!(res, 1.1);
    }

    #[test]
    fn test_bool_primitives_to_scalar() {
        let exp = Scalar::bool(true);

        let res: Scalar = true.into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from(true);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_bool_scalar_to_primitives() {
        let res: bool = Scalar::bool(true).into();
        assert_eq!(res, true);

        let res: bool = bool::from(Scalar::bool(true));
        assert_eq!(res, true);
    }

    #[test]
    fn test_str_primitives_to_scalar() {
        let exp = Scalar::String("a".to_string());

        let res: Scalar = "a".to_string().into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from("a".to_string());
        assert_eq!(res, exp);

        // &str
        // let res: Scalar = "a".into();
        // assert_eq!(res, exp);

        // let res: Scalar = Scalar::from("a");
        // assert_eq!(res, exp);
    }

    #[test]
    fn test_str_scalar_to_primitives() {
        let res: String = Scalar::String("a".to_string()).into();
        assert_eq!(res, "a".to_string());

        let res: String = String::from(Scalar::String("a".to_string()));
        assert_eq!(res, "a".to_string());
    }
}
