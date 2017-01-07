use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use nullvec::NullVec;
use vec_ops::Elemwise;
use traits::TypeDispatch;

fn get_new_mask(x: &Option<Vec<bool>>, y: &Option<Vec<bool>>) -> Option<Vec<bool>> {
    match (x, y) {
        (&Some(ref xmask), &Some(ref ymask)) => {
            Some(Elemwise::elemwise_rr(xmask, ymask, |x, y| x | y))
        }
        (_, &Some(ref ymask)) => Some(ymask.clone()),
        (&Some(ref xmask), _) => Some(xmask.clone()),
        (&None, &None) => None,
    }
}

macro_rules! add_nullvec_elemwise_op {
    ($t:ident, $tr:ident, $op:ident, $sym:tt) => {
        // Nullvec + NullVec
        impl $tr<NullVec<$t>> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: NullVec<$t>) -> NullVec<$t> {
                let new_mask = get_new_mask(&self.mask, &other.mask);
                let new_values = Elemwise::elemwise_oo(self.data, other.data, |x, y| x $sym y);
                NullVec::with_mask(new_values, new_mask)
            }
        }
        // Nullvec + &NullVec
        impl<'a> $tr<&'a NullVec<$t>> for NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &NullVec<$t>) -> NullVec<$t> {
                let new_mask = get_new_mask(&self.mask, &other.mask);
                let new_values = Elemwise::elemwise_or(self.data, &other.data, |x, y| x $sym y);
                NullVec::with_mask(new_values, new_mask)
            }
        }
        // &Nullvec + NullVec
        impl<'b> $tr<NullVec<$t>> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: NullVec<$t>) -> NullVec<$t> {
                let new_mask = get_new_mask(&self.mask, &other.mask);
                let new_values = Elemwise::elemwise_ro(&self.data, other.data, |x, y| x $sym y);
                NullVec::with_mask(new_values, new_mask)
            }
        }
        // &Nullvec + &NullVec
        impl<'a, 'b> $tr<&'a NullVec<$t>> for &'b NullVec<$t> {
            type Output = NullVec<$t>;
            fn $op(self, other: &NullVec<$t>) -> NullVec<$t> {
                let new_mask = get_new_mask(&self.mask, &other.mask);
                let new_values = Elemwise::elemwise_rr(&self.data, &other.data, |x, y| x $sym y);
                NullVec::with_mask(new_values, new_mask)
            }
        }
    }
}

macro_rules! add_nullvec_elemwise_arithmetic_op_patterns {
    ($t:ident) => {
        add_nullvec_elemwise_op!($t, Add, add, +);
        add_nullvec_elemwise_op!($t, Sub, sub, -);
        add_nullvec_elemwise_op!($t, Mul, mul, *);
        add_nullvec_elemwise_op!($t, Div, div, /);
        add_nullvec_elemwise_op!($t, Rem, rem, %);
    }
}
macro_dispatch!(add_nullvec_elemwise_arithmetic_op_patterns,
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

macro_rules! add_nullvec_elemwise_bitwise_op_patterns {
    ($t:ident) => {
        add_nullvec_elemwise_op!($t, BitAnd, bitand, &);
        add_nullvec_elemwise_op!($t, BitOr, bitor, |);
        add_nullvec_elemwise_op!($t, BitXor, bitxor, ^);
    }
}
macro_dispatch!(add_nullvec_elemwise_bitwise_op_patterns,
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
    use traits::TypeDispatch;

    #[test]
    fn test_int() {
        let v1: Vec<usize> = vec![1, 2, 3];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<usize> = vec![1, 2, 3];
        let nvec2 = NullVec::new(v2);

        let res = nvec1 + nvec2;
        assert_eq!(res.data, vec![2, 4, 6]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_rhs() {
        let v1: Vec<usize> = vec![1, 2, 3];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<usize> = vec![1, 2, 3];
        let nvec2 = NullVec::new(v2);

        let res = nvec1 + &nvec2;
        assert_eq!(res.data, vec![2, 4, 6]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_lhs() {
        let v1: Vec<usize> = vec![1, 2, 3];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<usize> = vec![1, 2, 3];
        let nvec2 = NullVec::new(v2);

        let res = &nvec1 + nvec2;
        assert_eq!(res.data, vec![2, 4, 6]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_int_ref_both() {
        let v1: Vec<usize> = vec![1, 2, 3];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<usize> = vec![1, 2, 3];
        let nvec2 = NullVec::new(v2);

        let res = &nvec1 + &nvec2;
        assert_eq!(res.data, vec![2, 4, 6]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float() {
        let v1: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec2 = NullVec::new(v2);

        let res = nvec1 + nvec2;
        assert_eq!(res.data, vec![2.2, 2.4, 2.6]);
        assert_eq!(res.mask, None);
    }

    #[test]
    fn test_float_nan_lhs() {
        let v1: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec2 = NullVec::new(v2);

        let res = nvec1 + nvec2;
        assert_eq!(res.data, vec![2.2, 1.2, 2.6]);
        assert_eq!(res.mask, Some(vec![false, true, false]));
    }

    #[test]
    fn test_float_nan_rhs() {
        let v1: Vec<f64> = vec![1.1, 1.2, 1.3];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec2 = NullVec::new(v2);

        let res = nvec1 + nvec2;
        assert_eq!(res.data, vec![2.2, 1.2, 2.6]);
        assert_eq!(res.mask, Some(vec![false, true, false]));
    }

    #[test]
    fn test_float_nan_both() {
        let v1: Vec<f64> = vec![1.1, 1.2, f64::NAN];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<f64> = vec![1.1, f64::NAN, 1.3];
        let nvec2 = NullVec::new(v2);

        let res = nvec1 + nvec2;
        assert_eq!(res.data, vec![2.2, 1.2, 1.3]);
        assert_eq!(res.mask, Some(vec![false, true, true]));
    }

    #[test]
    fn test_bool() {
        let v1: Vec<bool> = vec![true, false, true];
        let nvec1 = NullVec::new(v1);

        let v2: Vec<bool> = vec![true, true, true];
        let nvec2 = NullVec::new(v2);

        let res = nvec1 & nvec2;
        assert_eq!(res.data, vec![true, false, true]);
        assert_eq!(res.mask, None);
    }
}
