use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use algos::vec_ops::Elemwise;
use nullvec::NullVec;
use traits::VecBase;

macro_rules! add_vec_elemwise_op {
    ($t:ident, $tr:ident, $op:ident, $sym:tt) => {
        // Nullvec + Vec
        impl $tr<Vec<$t>> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: Vec<$t>) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::elemwise_oo(self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
        // Nullvec + &Vec
        impl<'a> $tr<&'a Vec<$t>> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &Vec<$t>) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::elemwise_or(self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
        // &Nullvec + Vec
        impl<'b> $tr<Vec<$t>> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: Vec<$t>) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::elemwise_ro(&self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
        // &Nullvec + &Vec
        impl<'a, 'b> $tr<&'a Vec<$t>> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &Vec<$t>) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::elemwise_rr(&self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
    }
}

macro_rules! add_vec_elemwise_arithmetic_op_patterns {
    ($t:ident) => {
        add_vec_elemwise_op!($t, Add, add, +);
        add_vec_elemwise_op!($t, Sub, sub, -);
        add_vec_elemwise_op!($t, Mul, mul, *);
        add_vec_elemwise_op!($t, Div, div, /);
        add_vec_elemwise_op!($t, Rem, rem, %);
    }
}
macro_dispatch!(add_vec_elemwise_arithmetic_op_patterns,
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

macro_rules! add_vec_elemwise_bitwise_op_patterns {
    ($t:ident) => {
        add_vec_elemwise_op!($t, BitAnd, bitand, &);
        add_vec_elemwise_op!($t, BitOr, bitor, |);
        add_vec_elemwise_op!($t, BitXor, bitxor, ^);
    }
}
macro_dispatch!(add_vec_elemwise_bitwise_op_patterns,
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

#[cfg(test)]
mod tests {

    use std::f64;

    use nullvec::NullVec;
    use traits::VecBase;

    #[test]
    fn test_int() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + vec![2, 2, 2];
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_rhs() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = nvec + &vec![2, 2, 2];
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_lhs() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + vec![2, 2, 2];
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_both() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        let res = &nvec + &vec![2, 2, 2];
        assert_eq!(res.data, vec![3, 4, 5]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float() {
        let values: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec = NullVec::new(values);

        let res = nvec + vec![2., 2., 2.];
        assert_eq!(res.data, vec![3.1, 3.2, 3.3]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float_nan() {
        let values: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec = NullVec::new(values);

        let res = nvec + vec![2., 2., 2.];
        assert_eq!(res.data, vec![3.1, 2., 3.3]);
        assert_eq!(res.mask, Some(vec![false, true, false]));
    }

    #[test]
    fn test_bool() {
        let values: Vec<bool> = vec![true, false, true];
        let nvec = NullVec::new(values);

        let res = nvec & vec![true, true, true];
        assert_eq!(res.data, vec![true, false, true]);
        assert_eq!(res.mask, None);
    }
}
