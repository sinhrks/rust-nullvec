
/// /////////////////////////////////////////////////////////////////////////////
/// Base for Type Dispatch
/// /////////////////////////////////////////////////////////////////////////////

/// Basic trait all nullable scalar-likes must support
pub trait ScalarBase<T>: Sized {
    fn new(values: T) -> Self;
}

/// Basic trait all nullable vector-likes must support
pub trait VecBase<T>: Sized {
    fn new(values: Vec<T>) -> Self;

    /// Return the same type filling with all null
    fn with_mask(values: Vec<T>, mask: Option<Vec<bool>>) -> Self;
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
