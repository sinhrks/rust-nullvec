
/// Basic trait which must be implemented to primitive types
/// being stored to Nullable and NullVec
pub trait NullStorable: Default {
    /// Whether the primitive value can be null.
    ///
    /// If this is `false`, `is_null` never return true.
    fn has_primitive_null() -> bool {
        false
    }

    /// Whether the value should be regarded as null.
    /// Must be inverse of `is_not_null`.
    fn is_null(&self) -> bool {
        false
    }

    /// Whether the value should be regarded as null.
    /// Must be inverse of `is_null`.
    fn is_not_null(&self) -> bool {
        true
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Indexing
/// /////////////////////////////////////////////////////////////////////////////

/// Indexing methods for non-labeled Array / Indexer
pub trait Slicer: Sized {
    type Scalar;

    /// Return the length of myself
    fn len(&self) -> usize;

    /// Return a single element specified with the location
    ///
    /// # Panics
    ///
    /// - if specified locations outs of bounds
    fn iloc(&self, location: &usize) -> Self::Scalar;

    /// Return a single element specified with the location
    unsafe fn iloc_unchecked(&self, location: &usize) -> Self::Scalar;

    /// Return multiple elements specified with the locations
    ///
    /// # Panics
    ///
    /// - if specified locations outs of bounds
    fn ilocs(&self, locations: &[usize]) -> Self;

    /// Return multiple elements specified with the locations
    unsafe fn ilocs_unchecked(&self, locations: &[usize]) -> Self;

    /// Return multiple elements specified with the locations
    ///
    /// If specified locations outs of bounds, corresponding element is filled with null
    fn ilocs_forced(&self, locations: &[usize]) -> Self;

    /// Return multilpe elements specified with bool flags
    fn blocs(&self, flags: &[bool]) -> Self;

    /// Return multiple elements specified with the locations
    fn reindex(&self, locations: &[usize]) -> Self {
        self.ilocs(locations)
    }

    unsafe fn reindex_unchecked(&self, locations: &[usize]) -> Self {
        self.ilocs_unchecked(locations)
    }

    fn reindex_forced(&self, locations: &[usize]) -> Self {
        self.ilocs_forced(locations)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Aggregation
/// /////////////////////////////////////////////////////////////////////////////

pub trait BasicAggregation {
    // result which can keep current dtype
    type Kept;
    // result for count (to usize or its container)
    type Counted;

    fn sum(&self) -> Self::Kept;
    fn count(&self) -> Self::Counted;
}

pub trait NumericAggregation {
    // result which is coerced (to f64 or its container)
    type Coerced;

    fn mean(&self) -> Self::Coerced;
    fn var(&self) -> Self::Coerced;
    fn unbiased_var(&self) -> Self::Coerced;
    fn std(&self) -> Self::Coerced;
    fn unbiased_std(&self) -> Self::Coerced;
}

pub trait ComparisonAggregation {
    type Kept;

    fn min(&self) -> Self::Kept;
    fn max(&self) -> Self::Kept;
}
