use generic::Scalar;
use nullvec::NullVec;
use traits::NullStorable;

// allow to store Scalar to NullVec
impl Default for Scalar {
    fn default() -> Scalar {
        Scalar::Null
    }
}

impl NullStorable for Scalar {
    fn has_primitive_null() -> bool {
        true
    }

    fn is_null(&self) -> bool {
        match self {
            &Scalar::Null => true,
            _ => false,
        }
    }
}


impl Into<Vec<Scalar>> for NullVec<Scalar> {
    fn into(self: NullVec<Scalar>) -> Vec<Scalar> {
        // ToDo: check has_primitive_null and return type null
        match &self.mask {
            &None => self.data,
            &Some(ref mask) => {
                self.data
                    .into_iter()
                    .zip(mask.iter())
                    .map(|(v, &m)| if m == true { Scalar::Null } else { v })
                    .collect()
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use nullvec::NullVec;
    use generic::Scalar;

    #[test]
    fn test_scalar_to_nullvec() {
        // test here to check nullvec internal
        let v = vec![Scalar::i64(1), Scalar::f64(2.)];
        let nv = NullVec::new(v);
        assert_eq!(nv.data, vec![Scalar::i64(1), Scalar::f64(2.)]);
        assert_eq!(nv.mask, None);

        let v = vec![Scalar::i64(1), Scalar::Null];
        let nv = NullVec::new(v);
        assert_eq!(nv.data, vec![Scalar::i64(1), Scalar::Null]);
        assert_eq!(nv.mask, Some(vec![false, true]));
    }

    #[test]
    fn test_scalar_conv_from_vec() {
        // test here to check nullvec internal
        let v = vec![Scalar::i64(1), Scalar::f64(2.)];
        let nv: NullVec<Scalar> = v.into();
        assert_eq!(nv.data, vec![Scalar::i64(1), Scalar::f64(2.)]);
        assert_eq!(nv.mask, None);
        let res: Vec<Scalar> = nv.into();
        assert_eq!(res, vec![Scalar::i64(1), Scalar::f64(2.)]);

        let v = vec![Scalar::i64(1), Scalar::Null];
        let nv: NullVec<Scalar> = v.into();
        assert_eq!(nv.data, vec![Scalar::i64(1), Scalar::Null]);
        assert_eq!(nv.mask, Some(vec![false, true]));
        let res: Vec<Scalar> = nv.into();
        assert_eq!(res, vec![Scalar::i64(1), Scalar::Null]);
    }
}
