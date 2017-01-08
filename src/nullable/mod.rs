use std::f64;
use std::f32;

mod nullable_nullable_ops;
mod nullable_primitive_ops;
use traits::TypeDispatchScalar;

#[derive(Debug, PartialEq)]
pub enum Nullable<T> {
    Value(T),
    Null,
}

impl<T> TypeDispatchScalar<T> for Nullable<T> {
    fn new(value: T) -> Self {
        Nullable::Value(value)
    }
}

// conversion

macro_rules! impl_from_never_nullable {
    ($t:ident) => {

        impl From<$t> for Nullable<$t> {
            fn from(value: $t) -> Self {
                Nullable::Value(value)
            }
        }

        impl From<Nullable<$t>> for $t {
            fn from(value: Nullable<$t>) -> $t {
                match value {
                    Nullable::Value(val) => val,
                    _ => panic!("Unable to convert NaN to int")
                }
            }
        }
    }
}
macro_dispatch!(impl_from_never_nullable,
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

macro_rules! impl_from_nullable {
    ($t:ident) => {

        impl From<$t> for Nullable<$t> {
            fn from(value: $t) -> Self {
                if value.is_nan() {
                    Nullable::Null
                } else {
                    Nullable::Value(value)
                }
            }
        }

        impl From<Nullable<$t>> for $t {
            fn from(value: Nullable<$t>) -> $t {
                match value {
                    Nullable::Value(val) => val,
                    Nullable::Null => $t::NAN
                }
            }
        }
    }
}
macro_dispatch!(impl_from_nullable, f64, f32);

// Eq

impl<T> Eq for Nullable<T> where T: Eq {}

#[cfg(test)]
mod tests {

    use std::f64;
    use super::Nullable;
    use traits::TypeDispatchScalar;

    #[test]
    fn test_int() {
        let i1 = Nullable::new(3);
        let i2 = Nullable::new(3);
        assert_eq!(i1, i1);
        assert_eq!(i1, i2);
    }

    #[test]
    fn test_int_conv_from() {
        let i = Nullable::<i64>::from(3);
        assert_eq!(i, Nullable::new(3));
        let o = i64::from(i);
        assert_eq!(o, 3);
    }

    #[test]
    fn test_float_conv_from() {
        let f = Nullable::<f64>::from(3.0);
        assert_eq!(f, Nullable::new(3.0));
        let o = f64::from(f);
        assert_eq!(o, 3.0);

        let nf = Nullable::<f64>::from(f64::NAN);
        assert_eq!(nf, Nullable::Null);
        let no = f64::from(nf);
        assert_eq!(no.is_nan(), true);
    }

    #[test]
    fn test_int_ops_add() {
        let i1 = Nullable::Value(3);
        let i2 = Nullable::Value(2);
        assert_eq!(i1 + i2, Nullable::Value(5));

        let i3 = Nullable::Value(3);
        assert_eq!(i3 + Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(Nullable::Null + i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(n + Nullable::Null, Nullable::Null);
    }
}
