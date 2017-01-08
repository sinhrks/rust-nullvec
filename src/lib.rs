extern crate num;

// macro must be defined first to be usable in other modules
#[macro_use]
mod macros;

// Nullable container
mod nullvec;
// Nullable scalar
mod nullable;

// common
mod vec_ops;
mod traits;

pub mod prelude;

// - Ops ToDo
//
// - Nullable + primitive (done)
// - Nullable + Nullable (done)
// - Nullable + NullVec
//
// - NullVec + primitive (done)
// - NullVec + Nullable (done)
// - NullVec + Vec (done)
// - NullVec + NullVec (done)
//
// - Conversion ToDo
//
// - float and Null
// - Nullable and Null
//
// - vec and NullVec
//
// - ToDo:
// - Add array and scalar
//
