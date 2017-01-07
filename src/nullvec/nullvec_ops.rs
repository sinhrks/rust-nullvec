use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor};

use nullvec::NullVec;
use vec_ops::Elemwise;
use traits::TypeDispatch;

macro_rules! add_broadcast_op {
    ($t:ident, $tr:ident, $op:ident, $sym:tt) => {

        // NullVec and Scalar op
        impl $tr<$t> for NullVec<$t> {
            type Output = NullVec<$t>;

            fn $op(self, other: $t) -> NullVec<$t> {
                let mask = self.mask.clone();
                let new_values = Elemwise::broadcast_oo(self.data, other, |x, y| x $sym y);
                NullVec::with_mask(new_values, mask)
            }
        }
    }
}

macro_rules! add_broadcast_op_patterns {
    ($t:ident) => {
        add_broadcast_op!($t, Add, add, +);
    }
}
macro_dispatch!(add_broadcast_op_patterns, i64, i32, i16, i8, isize,
                u64, u32, u16, u8, usize, f64, f32);


#[cfg(test)]
mod tests {

    use std::f64;

    use nullvec::NullVec;
    use traits::TypeDispatch;

    #[test]
    fn test_int() {
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);

        let res = nvec + 2;
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
}
