use nullvec::NullVec;

mod array_impl;
mod array_impl_aggregation;
mod array_impl_slicer;
mod convert;
mod scalar_impl;

/// Generic scalar which can contain arbitrary primitive types.
#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug)]
pub enum Scalar {
    /// Store `i64` value
    i64(i64),
    /// Store `i32` value
    i32(i32),
    /// Store `i16` value
    i16(i16),
    /// Store `i8` value
    i8(i8),
    /// Store `isize` value
    isize(isize),
    /// Store `u64` value
    u64(u64),
    /// Store `u32` value
    u32(u32),
    /// Store `u16` value
    u16(u16),
    /// Store `u8` value
    u8(u8),
    /// Store `usize` value
    usize(usize),
    /// Store `f64` value
    f64(f64),
    /// Store `f32` value
    f32(f32),
    /// Store `bool` value
    bool(bool),
    /// Store `String` value
    String(String),
    /// Store `Null`
    Null,
}

/// Generic array which can contain `NullVec` of primitive types.
#[derive(Clone, PartialEq, Debug)]
pub enum Array {
    /// Nullable `i64` array
    Int64Array(NullVec<i64>),
    /// Nullable `i32` array
    Int32Array(NullVec<i32>),
    /// Nullable `i16` array
    Int16Array(NullVec<i16>),
    /// Nullable `i8` array
    Int8Array(NullVec<i8>),
    /// Nullable `isize` array
    IsizeArray(NullVec<isize>),
    /// Nullable `u64` array
    UInt64Array(NullVec<u64>),
    /// Nullable `u32` array
    UInt32Array(NullVec<u32>),
    /// Nullable `u16` array
    UInt16Array(NullVec<u16>),
    /// Nullable `u8` array
    UInt8Array(NullVec<u8>),
    /// Nullable `usize` array
    UsizeArray(NullVec<usize>),
    /// Nullable `f64` array
    Float64Array(NullVec<f64>),
    /// Nullable `f32` array
    Float32Array(NullVec<f32>),
    /// Nullable `bool` array
    BoolArray(NullVec<bool>),
    /// Nullable `String` array
    StringArray(NullVec<String>),
}
