use super::{Array, Scalar};
use traits::Slicer;

impl Slicer for Array {
    type Scalar = Scalar;

    fn len(&self) -> usize {
        match self {
            &Array::Int64Array(ref vals) => vals.len(),
            &Array::Int32Array(ref vals) => vals.len(),
            &Array::Int16Array(ref vals) => vals.len(),
            &Array::Int8Array(ref vals) => vals.len(),
            &Array::IsizeArray(ref vals) => vals.len(),
            &Array::UInt64Array(ref vals) => vals.len(),
            &Array::UInt32Array(ref vals) => vals.len(),
            &Array::UInt16Array(ref vals) => vals.len(),
            &Array::UInt8Array(ref vals) => vals.len(),
            &Array::UsizeArray(ref vals) => vals.len(),
            &Array::Float64Array(ref vals) => vals.len(),
            &Array::Float32Array(ref vals) => vals.len(),
            &Array::BoolArray(ref vals) => vals.len(),
            &Array::StringArray(ref vals) => vals.len(),
        }
    }

    fn iloc(&self, location: &usize) -> Self::Scalar {
        match self {
            &Array::Int64Array(ref vals) => vals.iloc(location).into(),
            &Array::Int32Array(ref vals) => vals.iloc(location).into(),
            &Array::Int16Array(ref vals) => vals.iloc(location).into(),
            &Array::Int8Array(ref vals) => vals.iloc(location).into(),
            &Array::IsizeArray(ref vals) => vals.iloc(location).into(),
            &Array::UInt64Array(ref vals) => vals.iloc(location).into(),
            &Array::UInt32Array(ref vals) => vals.iloc(location).into(),
            &Array::UInt16Array(ref vals) => vals.iloc(location).into(),
            &Array::UInt8Array(ref vals) => vals.iloc(location).into(),
            &Array::UsizeArray(ref vals) => vals.iloc(location).into(),
            &Array::Float64Array(ref vals) => vals.iloc(location).into(),
            &Array::Float32Array(ref vals) => vals.iloc(location).into(),
            &Array::BoolArray(ref vals) => vals.iloc(location).into(),
            &Array::StringArray(ref vals) => vals.iloc(location).into(),
        }
    }

    unsafe fn iloc_unchecked(&self, location: &usize) -> Self::Scalar {
        match self {
            &Array::Int64Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::Int32Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::Int16Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::Int8Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::IsizeArray(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::UInt64Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::UInt32Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::UInt16Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::UInt8Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::UsizeArray(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::Float64Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::Float32Array(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::BoolArray(ref vals) => vals.iloc_unchecked(location).into(),
            &Array::StringArray(ref vals) => vals.iloc_unchecked(location).into(),
        }
    }

    fn ilocs(&self, locations: &[usize]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => Array::Int64Array(vals.ilocs(locations)),
            &Array::Int32Array(ref vals) => Array::Int32Array(vals.ilocs(locations)),
            &Array::Int16Array(ref vals) => Array::Int16Array(vals.ilocs(locations)),
            &Array::Int8Array(ref vals) => Array::Int8Array(vals.ilocs(locations)),
            &Array::IsizeArray(ref vals) => Array::IsizeArray(vals.ilocs(locations)),
            &Array::UInt64Array(ref vals) => Array::UInt64Array(vals.ilocs(locations)),
            &Array::UInt32Array(ref vals) => Array::UInt32Array(vals.ilocs(locations)),
            &Array::UInt16Array(ref vals) => Array::UInt16Array(vals.ilocs(locations)),
            &Array::UInt8Array(ref vals) => Array::UInt8Array(vals.ilocs(locations)),
            &Array::UsizeArray(ref vals) => Array::UsizeArray(vals.ilocs(locations)),
            &Array::Float64Array(ref vals) => Array::Float64Array(vals.ilocs(locations)),
            &Array::Float32Array(ref vals) => Array::Float32Array(vals.ilocs(locations)),
            &Array::BoolArray(ref vals) => Array::BoolArray(vals.ilocs(locations)),
            &Array::StringArray(ref vals) => Array::StringArray(vals.ilocs(locations)),
        }
    }

    unsafe fn ilocs_unchecked(&self, locations: &[usize]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => Array::Int64Array(vals.ilocs_unchecked(locations)),
            &Array::Int32Array(ref vals) => Array::Int32Array(vals.ilocs_unchecked(locations)),
            &Array::Int16Array(ref vals) => Array::Int16Array(vals.ilocs_unchecked(locations)),
            &Array::Int8Array(ref vals) => Array::Int8Array(vals.ilocs_unchecked(locations)),
            &Array::IsizeArray(ref vals) => Array::IsizeArray(vals.ilocs_unchecked(locations)),
            &Array::UInt64Array(ref vals) => Array::UInt64Array(vals.ilocs_unchecked(locations)),
            &Array::UInt32Array(ref vals) => Array::UInt32Array(vals.ilocs_unchecked(locations)),
            &Array::UInt16Array(ref vals) => Array::UInt16Array(vals.ilocs_unchecked(locations)),
            &Array::UInt8Array(ref vals) => Array::UInt8Array(vals.ilocs_unchecked(locations)),
            &Array::UsizeArray(ref vals) => Array::UsizeArray(vals.ilocs_unchecked(locations)),
            &Array::Float64Array(ref vals) => Array::Float64Array(vals.ilocs_unchecked(locations)),
            &Array::Float32Array(ref vals) => Array::Float32Array(vals.ilocs_unchecked(locations)),
            &Array::BoolArray(ref vals) => Array::BoolArray(vals.ilocs_unchecked(locations)),
            &Array::StringArray(ref vals) => Array::StringArray(vals.ilocs_unchecked(locations)),
        }
    }

    fn ilocs_forced(&self, locations: &[usize]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => Array::Int64Array(vals.ilocs_forced(locations)),
            &Array::Int32Array(ref vals) => Array::Int32Array(vals.ilocs_forced(locations)),
            &Array::Int16Array(ref vals) => Array::Int16Array(vals.ilocs_forced(locations)),
            &Array::Int8Array(ref vals) => Array::Int8Array(vals.ilocs_forced(locations)),
            &Array::IsizeArray(ref vals) => Array::IsizeArray(vals.ilocs_forced(locations)),
            &Array::UInt64Array(ref vals) => Array::UInt64Array(vals.ilocs_forced(locations)),
            &Array::UInt32Array(ref vals) => Array::UInt32Array(vals.ilocs_forced(locations)),
            &Array::UInt16Array(ref vals) => Array::UInt16Array(vals.ilocs_forced(locations)),
            &Array::UInt8Array(ref vals) => Array::UInt8Array(vals.ilocs_forced(locations)),
            &Array::UsizeArray(ref vals) => Array::UsizeArray(vals.ilocs_forced(locations)),
            &Array::Float64Array(ref vals) => Array::Float64Array(vals.ilocs_forced(locations)),
            &Array::Float32Array(ref vals) => Array::Float32Array(vals.ilocs_forced(locations)),
            &Array::BoolArray(ref vals) => Array::BoolArray(vals.ilocs_forced(locations)),
            &Array::StringArray(ref vals) => Array::StringArray(vals.ilocs_forced(locations)),
        }
    }

    fn blocs(&self, flags: &[bool]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => Array::Int64Array(vals.blocs(flags)),
            &Array::Int32Array(ref vals) => Array::Int32Array(vals.blocs(flags)),
            &Array::Int16Array(ref vals) => Array::Int16Array(vals.blocs(flags)),
            &Array::Int8Array(ref vals) => Array::Int8Array(vals.blocs(flags)),
            &Array::IsizeArray(ref vals) => Array::IsizeArray(vals.blocs(flags)),
            &Array::UInt64Array(ref vals) => Array::UInt64Array(vals.blocs(flags)),
            &Array::UInt32Array(ref vals) => Array::UInt32Array(vals.blocs(flags)),
            &Array::UInt16Array(ref vals) => Array::UInt16Array(vals.blocs(flags)),
            &Array::UInt8Array(ref vals) => Array::UInt8Array(vals.blocs(flags)),
            &Array::UsizeArray(ref vals) => Array::UsizeArray(vals.blocs(flags)),
            &Array::Float64Array(ref vals) => Array::Float64Array(vals.blocs(flags)),
            &Array::Float32Array(ref vals) => Array::Float32Array(vals.blocs(flags)),
            &Array::BoolArray(ref vals) => Array::BoolArray(vals.blocs(flags)),
            &Array::StringArray(ref vals) => Array::StringArray(vals.blocs(flags)),
        }
    }
}
