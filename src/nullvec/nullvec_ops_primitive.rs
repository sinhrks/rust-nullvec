use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use algos::vec_ops::Elemwise;
use nullvec::NullVec;

macro_rules! add_primitive_broadcast_op {
    ($t:ident, $tr:ident, $op:ident, $sym:tt) => {
        // Nullvec + Primitive
        impl $tr<$t> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: $t) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::broadcast_oo(self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
        // Nullvec + &Primitive
        impl<'a> $tr<&'a $t> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &$t) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::broadcast_or(self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
        // &Nullvec + Primitive
        impl<'b> $tr<$t> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: $t) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::broadcast_ro(&self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
        // &Nullvec + &Primitive
        impl<'a, 'b> $tr<&'a $t> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &$t) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::broadcast_rr(&self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
    }
}

macro_rules! add_primitive_broadcast_arithmetic_op_patterns {
    ($t:ident) => {
        add_primitive_broadcast_op!($t, Add, add, +);
        add_primitive_broadcast_op!($t, Sub, sub, -);
        add_primitive_broadcast_op!($t, Mul, mul, *);
        add_primitive_broadcast_op!($t, Div, div, /);
        add_primitive_broadcast_op!($t, Rem, rem, %);
    }
}
macro_dispatch!(
    add_primitive_broadcast_arithmetic_op_patterns,
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

macro_rules! add_primitive_broadcast_bitwise_op_patterns {
    ($t:ident) => {
        add_primitive_broadcast_op!($t, BitAnd, bitand, &);
        add_primitive_broadcast_op!($t, BitOr, bitor, |);
        add_primitive_broadcast_op!($t, BitXor, bitxor, ^);
    }
}
macro_dispatch!(
    add_primitive_broadcast_bitwise_op_patterns,
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

    #[test]
    fn test_int() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + 2;
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_rhs() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + &2;
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_lhs() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + 2;
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_both() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + &2;
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float() {
        let values: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec = NullVec::new(values);

        let res = nvec + 2.;
        assert_eq!(res.data, vec![3.1, 3.2, 3.3]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float_nan() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);

        let res = nvec + 2.;
        assert_eq!(res.data, vec![3.1, 2., 3.3]);
        assert_eq!(res.mask, Some(vec![false, true, false]));
    }

    #[test]
    fn test_bool() {
        let values: Vec<bool> = vec![true, false, true];
        let nvec = NullVec::new(values);

        let res = nvec & true;
        assert_eq!(res.data, vec![true, false, true]);
        assert_eq!(res.mask, None);
    }
}
