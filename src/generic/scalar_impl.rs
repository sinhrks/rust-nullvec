use super::Scalar;
use nullable::Nullable;

macro_rules! iml_scalar_as {
    ($t:ident, $op:ident) => {

        // ToDo: fix docstring
        impl Scalar {
            /// Convert the value to specified type
            ///
            /// The result may be undefined as the same rule as `as`
            pub fn $op(&self) -> Nullable<$t> {
                match self {
                    &Scalar::Null => Nullable::Null,
                    &Scalar::i64(val) => Nullable::Value(val as $t),
                    &Scalar::i32(val) => Nullable::Value(val as $t),
                    &Scalar::i16(val) => Nullable::Value(val as $t),
                    &Scalar::i8(val) => Nullable::Value(val as $t),
                    &Scalar::isize(val) => Nullable::Value(val as $t),
                    &Scalar::u64(val) => Nullable::Value(val as $t),
                    &Scalar::u32(val) => Nullable::Value(val as $t),
                    &Scalar::u16(val) => Nullable::Value(val as $t),
                    &Scalar::u8(val) => Nullable::Value(val as $t),
                    &Scalar::usize(val) => Nullable::Value(val as $t),
                    &Scalar::f64(val) => Nullable::Value(val as $t),
                    &Scalar::f32(val) => Nullable::Value(val as $t),
                    _ => panic!("cannot convert to specified type")
                }
            }
        }
    }
}
iml_scalar_as!(i64, as_i64);
iml_scalar_as!(i32, as_i32);
iml_scalar_as!(i16, as_i16);
iml_scalar_as!(i8, as_i8);
iml_scalar_as!(isize, as_isize);
iml_scalar_as!(u64, as_u64);
iml_scalar_as!(u32, as_u32);
iml_scalar_as!(u16, as_u16);
iml_scalar_as!(u8, as_u8);
iml_scalar_as!(usize, as_usize);
iml_scalar_as!(f64, as_f64);
iml_scalar_as!(f32, as_f32);

impl Scalar {
    pub fn as_bool(&self) -> Nullable<bool> {
        match self {
            &Scalar::Null => Nullable::Null,
            &Scalar::bool(val) => Nullable::Value(val),
            _ => panic!("cannot convert to specified type"),
        }
    }
}

impl Scalar {
    pub fn as_str(&self) -> Nullable<String> {
        match self {
            &Scalar::Null => Nullable::Null,
            &Scalar::String(ref val) => Nullable::Value(val.clone()),
            _ => panic!("cannot convert to specified type"),
        }
    }
}
