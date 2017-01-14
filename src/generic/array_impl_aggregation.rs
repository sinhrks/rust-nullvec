use super::{Array, Scalar};
use nullable::Nullable;
use traits::{BasicAggregation, NumericAggregation, ComparisonAggregation};

impl BasicAggregation for Array {
    type Kept = Scalar;
    type Counted = usize;

    fn sum(&self) -> Self::Kept {
        match self {
            &Array::Int64Array(ref vals) => vals.sum().into(),
            &Array::Int32Array(ref vals) => vals.sum().into(),
            &Array::Int16Array(ref vals) => vals.sum().into(),
            &Array::Int8Array(ref vals) => vals.sum().into(),
            &Array::IsizeArray(ref vals) => vals.sum().into(),
            &Array::UInt64Array(ref vals) => vals.sum().into(),
            &Array::UInt32Array(ref vals) => vals.sum().into(),
            &Array::UInt16Array(ref vals) => vals.sum().into(),
            &Array::UInt8Array(ref vals) => vals.sum().into(),
            &Array::UsizeArray(ref vals) => vals.sum().into(),
            &Array::Float64Array(ref vals) => vals.sum().into(),
            &Array::Float32Array(ref vals) => vals.sum().into(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }

    fn count(&self) -> Self::Counted {
        match self {
            &Array::Int64Array(ref vals) => vals.count(),
            &Array::Int32Array(ref vals) => vals.count(),
            &Array::Int16Array(ref vals) => vals.count(),
            &Array::Int8Array(ref vals) => vals.count(),
            &Array::IsizeArray(ref vals) => vals.count(),
            &Array::UInt64Array(ref vals) => vals.count(),
            &Array::UInt32Array(ref vals) => vals.count(),
            &Array::UInt16Array(ref vals) => vals.count(),
            &Array::UInt8Array(ref vals) => vals.count(),
            &Array::UsizeArray(ref vals) => vals.count(),
            &Array::Float64Array(ref vals) => vals.count(),
            &Array::Float32Array(ref vals) => vals.count(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }
}

impl NumericAggregation for Array {
    type Coerced = Nullable<f64>;

    fn mean(&self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => vals.mean(),
            &Array::Int32Array(ref vals) => vals.mean(),
            &Array::Int16Array(ref vals) => vals.mean(),
            &Array::Int8Array(ref vals) => vals.mean(),
            &Array::IsizeArray(ref vals) => vals.mean(),
            &Array::UInt64Array(ref vals) => vals.mean(),
            &Array::UInt32Array(ref vals) => vals.mean(),
            &Array::UInt16Array(ref vals) => vals.mean(),
            &Array::UInt8Array(ref vals) => vals.mean(),
            &Array::UsizeArray(ref vals) => vals.mean(),
            &Array::Float64Array(ref vals) => vals.mean(),
            &Array::Float32Array(ref vals) => vals.mean(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }

    fn var(&self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => vals.var(),
            &Array::Int32Array(ref vals) => vals.var(),
            &Array::Int16Array(ref vals) => vals.var(),
            &Array::Int8Array(ref vals) => vals.var(),
            &Array::IsizeArray(ref vals) => vals.var(),
            &Array::UInt64Array(ref vals) => vals.var(),
            &Array::UInt32Array(ref vals) => vals.var(),
            &Array::UInt16Array(ref vals) => vals.var(),
            &Array::UInt8Array(ref vals) => vals.var(),
            &Array::UsizeArray(ref vals) => vals.var(),
            &Array::Float64Array(ref vals) => vals.var(),
            &Array::Float32Array(ref vals) => vals.var(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }

    fn unbiased_var(&self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => vals.unbiased_var(),
            &Array::Int32Array(ref vals) => vals.unbiased_var(),
            &Array::Int16Array(ref vals) => vals.unbiased_var(),
            &Array::Int8Array(ref vals) => vals.unbiased_var(),
            &Array::IsizeArray(ref vals) => vals.unbiased_var(),
            &Array::UInt64Array(ref vals) => vals.unbiased_var(),
            &Array::UInt32Array(ref vals) => vals.unbiased_var(),
            &Array::UInt16Array(ref vals) => vals.unbiased_var(),
            &Array::UInt8Array(ref vals) => vals.unbiased_var(),
            &Array::UsizeArray(ref vals) => vals.unbiased_var(),
            &Array::Float64Array(ref vals) => vals.unbiased_var(),
            &Array::Float32Array(ref vals) => vals.unbiased_var(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }

    fn std(&self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => vals.std(),
            &Array::Int32Array(ref vals) => vals.std(),
            &Array::Int16Array(ref vals) => vals.std(),
            &Array::Int8Array(ref vals) => vals.std(),
            &Array::IsizeArray(ref vals) => vals.std(),
            &Array::UInt64Array(ref vals) => vals.std(),
            &Array::UInt32Array(ref vals) => vals.std(),
            &Array::UInt16Array(ref vals) => vals.std(),
            &Array::UInt8Array(ref vals) => vals.std(),
            &Array::UsizeArray(ref vals) => vals.std(),
            &Array::Float64Array(ref vals) => vals.std(),
            &Array::Float32Array(ref vals) => vals.std(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }

    fn unbiased_std(&self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => vals.unbiased_std(),
            &Array::Int32Array(ref vals) => vals.unbiased_std(),
            &Array::Int16Array(ref vals) => vals.unbiased_std(),
            &Array::Int8Array(ref vals) => vals.unbiased_std(),
            &Array::IsizeArray(ref vals) => vals.unbiased_std(),
            &Array::UInt64Array(ref vals) => vals.unbiased_std(),
            &Array::UInt32Array(ref vals) => vals.unbiased_std(),
            &Array::UInt16Array(ref vals) => vals.unbiased_std(),
            &Array::UInt8Array(ref vals) => vals.unbiased_std(),
            &Array::UsizeArray(ref vals) => vals.unbiased_std(),
            &Array::Float64Array(ref vals) => vals.unbiased_std(),
            &Array::Float32Array(ref vals) => vals.unbiased_std(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }
}

impl ComparisonAggregation for Array {
    type Kept = Scalar;

    fn min(&self) -> Self::Kept {
        match self {
            &Array::Int64Array(ref vals) => vals.min().into(),
            &Array::Int32Array(ref vals) => vals.min().into(),
            &Array::Int16Array(ref vals) => vals.min().into(),
            &Array::Int8Array(ref vals) => vals.min().into(),
            &Array::IsizeArray(ref vals) => vals.min().into(),
            &Array::UInt64Array(ref vals) => vals.min().into(),
            &Array::UInt32Array(ref vals) => vals.min().into(),
            &Array::UInt16Array(ref vals) => vals.min().into(),
            &Array::UInt8Array(ref vals) => vals.min().into(),
            &Array::UsizeArray(ref vals) => vals.min().into(),
            &Array::Float64Array(ref vals) => vals.min().into(),
            &Array::Float32Array(ref vals) => vals.min().into(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }

    fn max(&self) -> Self::Kept {
        match self {
            &Array::Int64Array(ref vals) => vals.max().into(),
            &Array::Int32Array(ref vals) => vals.max().into(),
            &Array::Int16Array(ref vals) => vals.max().into(),
            &Array::Int8Array(ref vals) => vals.max().into(),
            &Array::IsizeArray(ref vals) => vals.max().into(),
            &Array::UInt64Array(ref vals) => vals.max().into(),
            &Array::UInt32Array(ref vals) => vals.max().into(),
            &Array::UInt16Array(ref vals) => vals.max().into(),
            &Array::UInt8Array(ref vals) => vals.max().into(),
            &Array::UsizeArray(ref vals) => vals.max().into(),
            &Array::Float64Array(ref vals) => vals.max().into(),
            &Array::Float32Array(ref vals) => vals.max().into(),
            &Array::BoolArray(_) => unimplemented!(),
            &Array::StringArray(_) => unimplemented!(),
        }
    }
}
