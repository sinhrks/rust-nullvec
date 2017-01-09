use nullvec::NullVec;

mod convert;
mod scalar_impl;

/// /////////////////////////////////////////////////////////////////////////////
/// Scalar
/// /////////////////////////////////////////////////////////////////////////////
#[allow(non_camel_case_types)]
#[derive(RustcDecodable, RustcEncodable, Clone, PartialEq, Debug)]
pub enum Scalar {
    i64(i64),
    i32(i32),
    i16(i16),
    i8(i8),
    isize(isize),
    u64(u64),
    u32(u32),
    u16(u16),
    u8(u8),
    usize(usize),
    f64(f64),
    f32(f32),
    bool(bool),
    String(String),
    Null,
}

/// /////////////////////////////////////////////////////////////////////////////
/// Array
/// /////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Debug)]
pub enum Array {
    Int64Array(NullVec<i64>),
    Int32Array(NullVec<i32>),
    Int16Array(NullVec<i16>),
    Int8Array(NullVec<i8>),
    IsizeArray(NullVec<isize>),
    UInt64Array(NullVec<u64>),
    UInt32Array(NullVec<u32>),
    UInt16Array(NullVec<u16>),
    UInt8Array(NullVec<u8>),
    UsizeArray(NullVec<usize>),
    Float64Array(NullVec<f64>),
    Float32Array(NullVec<f32>),
    BoolArray(NullVec<bool>),
    StringArray(NullVec<String>),
}
