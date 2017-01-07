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





/*
- Ops ToDo
  - Null + Null
  - Null + primitive
  - Null + Nullable
  - Null + NullVec

  - Nullable + Null
  - Nullable + primitive
  - Nullable + Nullable
  - Nullable + NullVec

  - NullVec + Null
  - NullVec + primitive
  - NullVec + Nullable
  - NullVec + NullVec

- Conversion ToDo

  - float and Null
  - Nullable and Null

  - vec and NullVec

- ToDo:
  - Add array and scalar
 */
