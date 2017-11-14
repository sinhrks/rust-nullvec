use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use nullvec::NullVec;
use nullable::Nullable;

macro_rules! add_nullable_broadcast_op {
    ($t:ident, $tr:ident, $op:ident, $sym:tt) => {
        // Nullvec + Nullable
        impl $tr<Nullable<$t>> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: Nullable<$t>) -> NullVec<$t> {
                match other {
                    Nullable::Null => self.as_null(),
                    // call Nulvec + primitive op
                    Nullable::Value(val) => self $sym val
                }
            }
        }
        // Nullvec + &Nullable
        impl<'a> $tr<&'a Nullable<$t>> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &'a Nullable<$t>) -> NullVec<$t> {
                match other {
                    &Nullable::Null => self.as_null(),
                    &Nullable::Value(ref val) => self $sym val
                }
            }
        }
        // &Nullvec + Nullbale
        impl<'b> $tr<Nullable<$t>> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: Nullable<$t>) -> NullVec<$t> {
                match other {
                    Nullable::Null => self.as_null(),
                    Nullable::Value(val) => self $sym val
                }
            }
        }
        // &Nullvec + &Nullable
        impl<'a, 'b> $tr<&'a Nullable<$t>> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &'a Nullable<$t>) -> NullVec<$t> {
                match other {
                    &Nullable::Null => self.as_null(),
                    &Nullable::Value(ref val) => self $sym val
                }
            }
        }
    }
}

macro_rules! add_nullable_broadcast_arithmetic_op_patterns {
    ($t:ident) => {
        add_nullable_broadcast_op!($t, Add, add, +);
        add_nullable_broadcast_op!($t, Sub, sub, -);
        add_nullable_broadcast_op!($t, Mul, mul, *);
        add_nullable_broadcast_op!($t, Div, div, /);
        add_nullable_broadcast_op!($t, Rem, rem, %);
    }
}
macro_dispatch!(
    add_nullable_broadcast_arithmetic_op_patterns,
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
    f32
);

macro_rules! add_nullable_broadcast_bitwise_op_patterns {
    ($t:ident) => {
        add_nullable_broadcast_op!($t, BitAnd, bitand, &);
        add_nullable_broadcast_op!($t, BitOr, bitor, |);
        add_nullable_broadcast_op!($t, BitXor, bitxor, ^);
    }
}
macro_dispatch!(
    add_nullable_broadcast_bitwise_op_patterns,
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
    bool
);

#[cfg(test)]
mod tests {

    use std::f64;

    use nullvec::NullVec;
    use nullable::Nullable;

    #[test]
    fn test_int() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + Nullable::Value(2);
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + Nullable::Null;
        assert_eq!(res.data, vec![1, 2, 3]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }

    #[test]
    fn test_int_ref_rhs() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + &Nullable::Value(2);
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + &Nullable::Null;
        assert_eq!(res.data, vec![1, 2, 3]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }

    #[test]
    fn test_int_ref_lhs() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + Nullable::Value(2);
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + Nullable::Null;
        assert_eq!(res.data, vec![1, 2, 3]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }

    #[test]
    fn test_int_ref_both() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + &Nullable::Value(2);
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + &Nullable::Null;
        assert_eq!(res.data, vec![1, 2, 3]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }

    #[test]
    fn test_float() {
        let values: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec = NullVec::new(values);
        let res = nvec + Nullable::Value(2.);
        assert_eq!(res.data, vec![3.1, 3.2, 3.3]);
        assert_eq!(res.mask, None);

        let values: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec = NullVec::new(values);
        let res = nvec + Nullable::Null;
        assert_eq!(res.data, vec![1.1, 1.2, 1.3]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }

    #[test]
    fn test_float_nan() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);
        let res = nvec + Nullable::Value(2.);
        assert_eq!(res.data, vec![3.1, 2., 3.3]);
        assert_eq!(res.mask, Some(vec![false, true, false]));

        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);
        let res = nvec + Nullable::Null;
        assert_eq!(res.data, vec![1.1, 0., 1.3]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }

    #[test]
    fn test_bool() {
        let values: Vec<bool> = vec![true, false, true];
        let nvec = NullVec::new(values);
        let res = nvec & Nullable::Value(true);
        assert_eq!(res.data, vec![true, false, true]);
        assert_eq!(res.mask, None);

        let values: Vec<bool> = vec![true, false, true];
        let nvec = NullVec::new(values);
        let res = nvec & Nullable::Null;
        assert_eq!(res.data, vec![true, false, true]);
        assert_eq!(res.mask, Some(vec![true, true, true]));
    }
}
