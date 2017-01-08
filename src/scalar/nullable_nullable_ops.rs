use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use scalar::Nullable;

macro_rules! add_nullable_op {
    ($t:ident, $tr:ident, $op:ident) => {
        // Nullable + Nullable
        impl $tr<Nullable<$t>> for Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: Nullable<$t>) -> Nullable<$t> {
                match other {
                    Nullable::Null => Nullable::Null,
                    Nullable::Value(v2) => self.$op(v2)
                }
            }
        }
        // Nullable + &Nullable
        impl<'a> $tr<&'a Nullable<$t>> for Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: &'a Nullable<$t>) -> Nullable<$t> {
            match other {
                &Nullable::Null => Nullable::Null,
                &Nullable::Value(ref v2) => self.$op(v2)
            }
            }
        }
        // &Nullable + Nullable
        impl<'b> $tr<Nullable<$t>> for &'b Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: Nullable<$t>) -> Nullable<$t> {
                match other {
                    Nullable::Null => Nullable::Null,
                    Nullable::Value(v2) => self.$op(v2)
                }
            }
        }
        // &Nullable + &Nullable
        impl<'a, 'b> $tr<&'a Nullable<$t>> for &'b Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: &'a Nullable<$t>) -> Nullable<$t> {
                match other {
                    &Nullable::Null => Nullable::Null,
                    &Nullable::Value(ref v2) => self.$op(v2)
                }
            }
        }
    }
}
macro_rules! add_nullable_arithmatic_op_patterns {
    ($t:ident) => {
        add_nullable_op!($t, Add, add);
        add_nullable_op!($t, Sub, sub);
        add_nullable_op!($t, Mul, mul);
        add_nullable_op!($t, Div, div);
        add_nullable_op!($t, Rem, rem);
    }
}
macro_dispatch!(add_nullable_arithmatic_op_patterns,
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
                f32);

macro_rules! add_nullable_bitwise_op_patterns {
    ($t:ident) => {
        add_nullable_op!($t, BitAnd, bitand);
        add_nullable_op!($t, BitOr, bitor);
        add_nullable_op!($t, BitXor, bitxor);
    }
}
macro_dispatch!(add_nullable_bitwise_op_patterns,
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
                bool);

// ToDo: assign ops


#[cfg(test)]
mod tests {

    use scalar::Nullable;

    #[test]
    fn test_int_ops_add_nullable() {
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

    #[test]
    fn test_int_ops_sub_nullable() {
        let i1 = Nullable::Value(3);
        let i2 = Nullable::Value(2);
        assert_eq!(i1 - i2, Nullable::Value(1));

        let i3 = Nullable::Value(3);
        assert_eq!(i3 - Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(Nullable::Null - i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(n - Nullable::Null, Nullable::Null);
    }

    #[test]
    fn test_int_ops_mul_nullable() {
        let i1 = Nullable::Value(3);
        let i2 = Nullable::Value(2);
        assert_eq!(i1 * i2, Nullable::Value(6));

        let i3 = Nullable::Value(3);
        assert_eq!(i3 * Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(Nullable::Null * i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(n * Nullable::Null, Nullable::Null);
    }

    #[test]
    fn test_int_ops_div_nullable() {
        let i1 = Nullable::Value(5);
        let i2 = Nullable::Value(2);
        assert_eq!(i1 / i2, Nullable::Value(2));

        let i3 = Nullable::Value(3);
        assert_eq!(i3 / Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(Nullable::Null / i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(n / Nullable::Null, Nullable::Null);
    }

    #[test]
    fn test_int_ops_rem_nullable() {
        let i1 = Nullable::Value(5);
        let i2 = Nullable::Value(3);
        assert_eq!(i1 % i2, Nullable::Value(2));

        let i3 = Nullable::Value(3);
        assert_eq!(i3 % Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(Nullable::Null % i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(n % Nullable::Null, Nullable::Null);
    }

    #[test]
    fn test_int_ops_ref_rhs_nullable() {
        let i1 = Nullable::Value(3);
        let i2 = Nullable::Value(2);
        assert_eq!(i1 + &i2, Nullable::Value(5));

        let i3 = Nullable::Value(3);
        assert_eq!(i3 + &Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(Nullable::Null + &i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(n + &Nullable::Null, Nullable::Null);
    }

    #[test]
    fn test_int_ops_ref_lhs_nullable() {
        let i1 = Nullable::Value(3);
        let i2 = Nullable::Value(2);
        assert_eq!(&i1 + i2, Nullable::Value(5));

        let i3 = Nullable::Value(3);
        assert_eq!(&i3 + Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(&Nullable::Null + i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(&n + Nullable::Null, Nullable::Null);
    }

    #[test]
    fn test_int_ops_ref_both_nullable() {
        let i1 = Nullable::Value(3);
        let i2 = Nullable::Value(2);
        assert_eq!(&i1 + &i2, Nullable::Value(5));

        let i3 = Nullable::Value(3);
        assert_eq!(&i3 + &Nullable::Null, Nullable::Null);
        let i4 = Nullable::Value(3);
        assert_eq!(&Nullable::Null + &i4, Nullable::Null);

        let n: Nullable<i64> = Nullable::Null;
        assert_eq!(&n + &Nullable::Null, Nullable::Null);
    }
}
