use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use nullable::Nullable;

macro_rules! add_primitive_op {
    ($t:ident, $tr:ident, $op:ident) => {
        // ToDo: type based exception logics, like 0 div and float nan
        // Nullable + Primitive
        impl $tr<$t> for Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: $t) -> Nullable<$t> {
                match self {
                    Nullable::Null => Nullable::Null,
                    Nullable::Value(v1) => Nullable::new(v1.$op(other))
                }
            }
        }
        // Nullable + &Primitive
        impl<'a> $tr<&'a $t> for Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: &'a $t) -> Nullable<$t> {
                match self {
                    Nullable::Null => Nullable::Null,
                    Nullable::Value(ref v1) => Nullable::new(v1.$op(*other))
                }
            }
        }
        // &Nullable + Primitive
        impl<'b> $tr<$t> for &'b Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: $t) -> Nullable<$t> {
                match self {
                    &Nullable::Null => Nullable::Null,
                    &Nullable::Value(ref v1) => Nullable::new(v1.$op(other))
                }
            }
        }
        // &Nullable + &Primitive
        impl<'a, 'b> $tr<&'a $t> for &'b Nullable<$t> {
            type Output = Nullable<$t>;
            fn $op(self, other: &'a $t) -> Nullable<$t> {
                match self {
                    &Nullable::Null => Nullable::Null,
                    &Nullable::Value(ref v1) => Nullable::new(v1.$op(*other))
                }
            }
        }
    }
}
macro_rules! add_primitive_arithmatic_op_patterns {
    ($t:ident) => {
        add_primitive_op!($t, Add, add);
        add_primitive_op!($t, Sub, sub);
        add_primitive_op!($t, Mul, mul);
        add_primitive_op!($t, Div, div);
        add_primitive_op!($t, Rem, rem);
    }
}
macro_dispatch!(
    add_primitive_arithmatic_op_patterns,
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

macro_rules! add_primitive_bitwise_op_patterns {
    ($t:ident) => {
        add_primitive_op!($t, BitAnd, bitand);
        add_primitive_op!($t, BitOr, bitor);
        add_primitive_op!($t, BitXor, bitxor);
    }
}
macro_dispatch!(
    add_primitive_bitwise_op_patterns,
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

    use nullable::Nullable;

    #[test]
    fn test_int_arithmetic_ops_add_primitive() {
        let i1 = Nullable::Value(3);
        assert_eq!(i1 + 2, Nullable::Value(5));
        assert_eq!(Nullable::Null + 2, Nullable::Null);

        let i1 = Nullable::Value(3);
        assert_eq!(i1 + &2, Nullable::Value(5));
        assert_eq!(Nullable::Null + &2, Nullable::Null);

        let i1 = Nullable::Value(3);
        assert_eq!(&i1 + 2, Nullable::Value(5));
        assert_eq!(&Nullable::Null + 2, Nullable::Null);

        let i1 = Nullable::Value(3);
        assert_eq!(&i1 + &2, Nullable::Value(5));
        assert_eq!(&Nullable::Null + &2, Nullable::Null);
    }

    #[test]
    fn test_int_bitwise_ops_add_primitive() {
        let i1 = Nullable::Value(true);
        assert_eq!(i1 & false, Nullable::Value(false));
        assert_eq!(Nullable::Null & false, Nullable::Null);

        let i1 = Nullable::Value(true);
        assert_eq!(i1 & &false, Nullable::Value(false));
        assert_eq!(Nullable::Null & &false, Nullable::Null);

        let i1 = Nullable::Value(true);
        assert_eq!(&i1 & false, Nullable::Value(false));
        assert_eq!(&Nullable::Null & false, Nullable::Null);

        let i1 = Nullable::Value(true);
        assert_eq!(&i1 & &false, Nullable::Value(false));
        assert_eq!(&Nullable::Null & &false, Nullable::Null);
    }
}
