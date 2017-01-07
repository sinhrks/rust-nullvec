extern crate num;

// macro must be defined first to be usable in other modules
#[macro_use]
mod macros;

// Null singleton
mod null;
// Nullable container
mod nullvec;
// Nullable scalar
mod scalar;

// common
mod vec_ops;
mod traits;





// - Ops ToDo
// - Null + Null
// - Null + primitive
// - Null + Nullable
// - Null + NullVec
//
// - Nullable + Null
// - Nullable + primitive (done)
// - Nullable + Nullable (done)
// - Nullable + NullVec
//
// - NullVec + Null
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
