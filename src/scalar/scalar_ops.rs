use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use scalar::Nullable;
use traits::TypeDispatchScalar;

macro_rules! add_op {
    ($tr:ident, $op:ident) => {

        impl<T> $tr<Nullable<T>> for Nullable<T>
            where T: $tr<T, Output=T> {
            type Output = Nullable<T>;

            fn $op(self, other: Nullable<T>) -> Nullable<T> {
                match (self, other) {
                    (Nullable::Null, _) => Nullable::Null,
                    (_, Nullable::Null) => Nullable::Null,
                    (Nullable::Value(v1), Nullable::Value(v2)) => Nullable::new(v1.$op(v2))
                }
            }
        }

        // ToDo: Can Copy be removed?
        impl<'a, T: Copy> $tr<&'a Nullable<T>> for Nullable<T>
            where T: $tr<T, Output=T> {
            type Output = Nullable<T>;

            fn $op(self, other: &'a Nullable<T>) -> Nullable<T> {
                match (self, other) {
                    (Nullable::Null, _) => Nullable::Null,
                    (_, &Nullable::Null) => Nullable::Null,
                    (Nullable::Value(ref v1), &Nullable::Value(ref v2)) => {
                        Nullable::new(v1.$op(*v2))
                    }
                }
            }
        }

        impl<'b, T: Copy> $tr<Nullable<T>> for &'b Nullable<T>
            where T: $tr<T, Output=T> {
            type Output = Nullable<T>;

            fn $op(self, other: Nullable<T>) -> Nullable<T> {
                match (self, other) {
                    (&Nullable::Null, _) => Nullable::Null,
                    (_, Nullable::Null) => Nullable::Null,
                    (&Nullable::Value(ref v1), Nullable::Value(ref v2)) => {
                        Nullable::new(v1.$op(*v2))
                    }
                }
            }
        }

        impl<'a, 'b, T: Copy> $tr<&'a Nullable<T>> for &'b Nullable<T>
            where T: $tr<T, Output=T> {
            type Output = Nullable<T>;

            fn $op(self, other: &'a Nullable<T>) -> Nullable<T> {
                match (self, other) {
                    (&Nullable::Null, _) => Nullable::Null,
                    (_, &Nullable::Null) => Nullable::Null,
                    (&Nullable::Value(ref v1), &Nullable::Value(ref v2)) => {
                        Nullable::new(v1.$op(*v2))
                    }
                }
            }
        }
    }
}
add_op!(Add, add);
add_op!(Sub, sub);
add_op!(Mul, mul);
add_op!(Div, div);
add_op!(Rem, rem);
add_op!(BitAnd, bitand);
add_op!(BitOr, bitor);
add_op!(BitXor, bitxor);

// ToDo: assign ops


#[cfg(test)]
mod tests {

    use scalar::Nullable;

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

    #[test]
    fn test_int_ops_sub() {
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
    fn test_int_ops_mul() {
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
    fn test_int_ops_div() {
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
    fn test_int_ops_rem() {
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
    fn test_int_ops_ref_rhs() {
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
    fn test_int_ops_ref_lhs() {
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
    fn test_int_ops_ref_both() {
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
