//! Nullable scalar which can contains specified type `T` and `Null`.
//!
//! # Examples
//!
//! ```
//! use nullvec::prelude::*;
//! let v = Nullable::new(1);
//! assert_eq!(v + Nullable::Null, Nullable::Null);
//! ```

use std::f64;
use std::f32;

mod nullable_nullable_ops;
mod nullable_primitive_ops;
use traits::NullStorable;

/// Nullable Scalar
#[derive(Debug, PartialEq)]
pub enum Nullable<T: NullStorable> {
    /// Value which is not `Null`
    Value(T),
    /// `Null`
    Null,
}

impl NullStorable for i64 {}
impl NullStorable for i32 {}
impl NullStorable for i16 {}
impl NullStorable for i8 {}
impl NullStorable for isize {}
impl NullStorable for u64 {}
impl NullStorable for u32 {}
impl NullStorable for u16 {}
impl NullStorable for u8 {}
impl NullStorable for usize {}
impl NullStorable for bool {}
impl NullStorable for String {}
impl NullStorable for f64 {
    fn has_primitive_null() -> bool {
        true
    }

    fn is_null(&self) -> bool {
        self.is_nan()
    }

    fn is_not_null(&self) -> bool {
        !self.is_nan()
    }
}

impl NullStorable for f32 {
    fn has_primitive_null() -> bool {
        true
    }

    fn is_null(&self) -> bool {
        self.is_nan()
    }

    fn is_not_null(&self) -> bool {
        !self.is_nan()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Basic impl
////////////////////////////////////////////////////////////////////////////////

impl<T: NullStorable> Nullable<T> {

    /// Create new `Nullable<T>` from `T`.
    ///
    /// Float `NAN` is automatically replaced to `Null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use nullvec::prelude::*;
    ///
    /// let n = Nullable::new(1);
    /// assert_eq!(&n + 1, Nullable::new(2));
    /// assert_eq!(&n + Nullable::Null, Nullable::Null);
    ///
    /// let n = Nullable::new(f64::NAN);
    /// assert_eq!(n, Nullable::Null);
    /// ```
    pub fn new(value: T) -> Self {
        if value.is_null() {
            Nullable::Null
        } else {
            Nullable::Value(value)
        }
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Basic conversion
/// /////////////////////////////////////////////////////////////////////////////

impl<T: NullStorable> From<T> for Nullable<T> {
    fn from(value: T) -> Self {
        Nullable::new(value)
    }
}

macro_rules! impl_from_never_nullable {
    ($t:ident) => {

        impl From<Nullable<$t>> for $t {
            fn from(value: Nullable<$t>) -> $t {
                match value {
                    Nullable::Value(val) => val,
                    // ToDo: must be TryFrom
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

impl<T: NullStorable> Eq for Nullable<T> where T: Eq {}

#[cfg(test)]
mod tests {

    use std::f64;
    use super::Nullable;

    #[test]
    fn test_int() {
        let i1 = Nullable::new(3);
        let i2 = Nullable::new(3);
        assert_eq!(i1, i1);
        assert_eq!(i1, i2);
        assert_eq!(i1, Nullable::new(3));

        assert!(i1 != Nullable::Null);
    }

    #[test]
    fn test_float() {
        let i1 = Nullable::new(1.1);
        let i2 = Nullable::new(f64::NAN);
        assert_eq!(i1, Nullable::Value(1.1));
        assert_eq!(i2, Nullable::Null);
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
