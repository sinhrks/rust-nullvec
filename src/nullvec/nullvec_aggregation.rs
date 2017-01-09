use num::{Zero, ToPrimitive};
use std::ops::{Add, Sub, Div};

use super::NullVec;
use nullable::Nullable;
use traits::{NullStorable, Slicer, BasicAggregation, NumericAggregation, ComparisonAggregation};


impl<T> BasicAggregation for NullVec<T>
    where T: Clone + Zero + Add + NullStorable
{
    type Kept = Nullable<T>;
    type Counted = usize;

    fn sum(&self) -> Self::Kept {
        let mut sum = T::zero();
        if self.len() == 0 {
            Nullable::Value(sum)
        } else {
            let mut has_value = false;
            for v in self.iter_not_null() {
                sum = sum + v.clone();
                has_value = true;
            }
            if has_value == true {
                Nullable::Value(sum)
            } else {
                Nullable::Null
            }
        }
    }

    fn count(&self) -> Self::Counted {
        let mut count = 0usize;
        match self.mask {
            Some(ref mask) => {
                for _ in self.iter_not_null() {
                    count += 1;
                }
                count
            }
            None => self.data.len(),
        }
    }
}

fn mean_sq<T>(not_null: Vec<T>) -> f64
    where T: Clone + ToPrimitive
{
    let not_null_f64: Vec<f64> = not_null.iter()
        .map(|x| ToPrimitive::to_f64(x).unwrap())
        .collect();
    // use two pass algorithm, assuming data is not large
    let mean: f64 = not_null_f64.iter()
        .fold(0., |a, b| a + b) / (not_null_f64.len() as f64);
    not_null_f64.into_iter()
        .fold(0., |a, b| a + (b - mean) * (b - mean))
}


impl<T> NumericAggregation for NullVec<T>
    where T: Clone + Zero + Add + Sub + Div + ToPrimitive + NullStorable
{
    type Coerced = Nullable<f64>;

    fn mean(&self) -> Self::Coerced {
        if self.len() == 0 {
            return Nullable::Null;
        }
        let sum = self.sum();
        match sum {
            Nullable::Value(ref val) => {
                Nullable::Value(ToPrimitive::to_f64(val).unwrap() / self.count() as f64)
            }
            Nullable::Null => Nullable::Null,
        }
    }

    fn var(&self) -> Self::Coerced {
        let not_null = self.not_null_values();
        let len: f64 = not_null.len() as f64;
        if not_null.len() == 0 {
            Nullable::Null
        } else {
            let msq = mean_sq(not_null);
            Nullable::Value(msq / len)
        }
    }

    fn unbiased_var(&self) -> Self::Coerced {
        let not_null = self.not_null_values();
        if not_null.len() == 0 {
            Nullable::Null
        } else {
            let len: f64 = (not_null.len() - 1) as f64;
            let msq = mean_sq(not_null);
            Nullable::Value(msq / len)
        }
    }

    fn std(&self) -> Self::Coerced {
        match self.var() {
            Nullable::Value(val) => Nullable::Value(val.sqrt()),
            Nullable::Null => Nullable::Null,
        }
    }

    fn unbiased_std(&self) -> Self::Coerced {
        match self.unbiased_var() {
            Nullable::Value(val) => Nullable::Value(val.sqrt()),
            Nullable::Null => Nullable::Null,
        }
    }
}

impl<T> ComparisonAggregation for NullVec<T>
    where T: Clone + PartialOrd + NullStorable
{
    type Kept = Nullable<T>;

    fn min(&self) -> Self::Kept {
        if self.len() == 0 {
            Nullable::Null
        } else {
            let mut it = self.iter_not_null().cloned();
            let mut current: T = match it.next() {
                Some(val) => val,
                None => return Nullable::Null,
            };
            for v in it {
                if v < current {
                    current = v;
                }
            }
            Nullable::Value(current)
        }
    }

    fn max(&self) -> Self::Kept {
        if self.len() == 0 {
            Nullable::Null
        } else {
            let mut it = self.iter_not_null().cloned();
            let mut current: T = match it.next() {
                Some(val) => val,
                None => return Nullable::Null,
            };
            for v in it {
                if v > current {
                    current = v;
                }
            }
            Nullable::Value(current)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::f64;

    use nullable::Nullable;
    use nullvec::NullVec;
    use traits::{Slicer, BasicAggregation, NumericAggregation, ComparisonAggregation};

    #[test]
    fn test_sum() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.sum(), Nullable::Value(0));

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.sum(), Nullable::Value(6));

        // skip null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.sum(), Nullable::Value(5));

        // all null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.sum(), Nullable::Null);
    }

    #[test]
    fn test_count() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.count(), 0);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.count(), 3);

        // skip null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.count(), 2);

        // all null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.count(), 0);
    }

    #[test]
    fn test_mean() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.mean(), Nullable::Null);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.mean(), Nullable::Value(2.));

        // skip null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.mean(), Nullable::Value(2.5));

        // all null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.mean(), Nullable::Null);
    }

    #[test]
    fn test_var() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.var(), Nullable::Null);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.var(), Nullable::Value(2. / 3.));

        // skip null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.var(), Nullable::Value(0.25));

        // all null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.var(), Nullable::Null);
    }

    #[test]
    fn test_unbiased_var() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.unbiased_var(), Nullable::Null);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.unbiased_var(), Nullable::Value(1.));

        // skip null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.unbiased_var(), Nullable::Value(0.5));

        // all null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.unbiased_var(), Nullable::Null);
    }

    #[test]
    fn test_std() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.std(), Nullable::Null);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.std(), Nullable::Value(0.81649658092772603));

        // skip null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.std(), Nullable::Value(0.5));

        // all null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.std(), Nullable::Null);
    }

    #[test]
    fn test_unbiased_std() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.unbiased_std(), Nullable::Null);

        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.unbiased_std(), Nullable::Value(1.));

        // skip null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, false, false]));
        assert_eq!(nvec.unbiased_std(), Nullable::Value(0.70710678118654757));

        // all null
        let values: Vec<usize> = vec![1, 2, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.unbiased_std(), Nullable::Null);
    }

    #[test]
    fn test_min() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.min(), Nullable::Null);

        let values: Vec<usize> = vec![2, 1, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.min(), Nullable::Value(1));

        // skip null
        let values: Vec<usize> = vec![2, 1, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, true, false]));
        assert_eq!(nvec.min(), Nullable::Value(2));

        // all null
        let values: Vec<usize> = vec![2, 1, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.min(), Nullable::Null);
    }

    #[test]
    fn test_max() {
        // length=0
        let values: Vec<usize> = vec![];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.max(), Nullable::Null);

        let values: Vec<usize> = vec![2, 1, 3];
        let nvec = NullVec::new(values);
        assert_eq!(nvec.max(), Nullable::Value(3));

        // skip null
        let values: Vec<usize> = vec![2, 1, 3];
        let nvec = NullVec::with_mask(values, Some(vec![false, true, false]));
        assert_eq!(nvec.max(), Nullable::Value(3));

        // all null
        let values: Vec<usize> = vec![2, 1, 3];
        let nvec = NullVec::with_mask(values, Some(vec![true, true, true]));
        assert_eq!(nvec.max(), Nullable::Null);
    }
}
