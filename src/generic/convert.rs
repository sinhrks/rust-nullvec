use super::{Array, Scalar};
use nullable::Nullable;
use nullvec::NullVec;

// ToDo: check all impls are enough efficient

// Vec to Array
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
impl<'a> From<Vec<&'a str>> for Array {
    fn from(values: Vec<&str>) -> Self {
        let string: Vec<String> = values.iter().map(|x| x.to_string()).collect();
        string.into()
    }
}

// Scalar Vec to Array
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

/// Scalar to primitives
/// /////////////////////////////////////////////////////////////////////////////

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



// This is the same than above but without the From<Scalar>
// Used only for String to avoid collisions with other conversions
macro_rules! add_scalar_conversion_str {
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
// String does not add a From<Scalar> impl to not conflict with the impls below
add_scalar_conversion_str!(String);

// &str and String conversions
// Libraries serializing Scalars will first create strings from them before writing them out

impl<'a> From<&'a str> for Scalar {
    fn from(value: &'a str) -> Self {
        if value == "Null" {
            Scalar::Null
        } else {
            value.parse().map(Scalar::i64)
                .or_else(|_| value.parse().map(Scalar::f64)
                         .or_else(|_| value.parse().map(Scalar::bool)))
                .unwrap_or_else(|_| Scalar::String(value.to_string()))
        }
    }
}

impl From<Scalar> for String {
    fn from(value: Scalar) -> Self {
        match value {
            Scalar::Null => "Null".to_string(),
            Scalar::String(val) => val,
            _ => value.to_string()
        }
    }
}


