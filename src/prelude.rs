
pub use nullvec::NullVec;
pub use nullable::Nullable;
pub use nullable::Nullable::Null;

pub use generic::{Array, Scalar};

pub use traits::{Slicer, BasicAggregation, NumericAggregation, ComparisonAggregation};

/// Module used for other package developers.
/// Users doesn't need to use it.
pub mod dev {
    pub use traits::NullStorable;

    pub mod algos {
        pub use algos::indexing::Indexing;
        pub use algos::sort::Sorter;
        pub use algos::vec_ops::Elemwise;
    }
}
