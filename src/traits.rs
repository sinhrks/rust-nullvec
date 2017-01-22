
/// Basic trait which must be implemented to primitive types
/// being stored to Nullable and NullVec
///
/// This trait is to store custom struct to `NullVec`. Normal users
/// do not need to import it.
pub trait NullStorable: Default {
    /// Whether the primitive value can be `Null`.
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
        !self.is_null()
    }
}

/// Indexing methods for 1-dimensional array-likes.
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
    ///
    /// # Panics
    ///
    /// - if specified locations outs of bounds
    fn reindex(&self, locations: &[usize]) -> Self {
        self.ilocs(locations)
    }

    /// Return multiple elements specified with the locations
    unsafe fn reindex_unchecked(&self, locations: &[usize]) -> Self {
        self.ilocs_unchecked(locations)
    }

    /// Return multiple elements specified with the locations
    ///
    /// If specified locations outs of bounds, corresponding element is filled with null
    fn reindex_forced(&self, locations: &[usize]) -> Self {
        self.ilocs_forced(locations)
    }
}

/// Stringify myself
pub trait Stringify {
    fn into_string_vec(&self) -> Vec<String>;
}

/// Basic aggregation methods
pub trait BasicAggregation {
    // result which can keep current dtype
    type Kept;
    // result for count (to usize or its container)
    type Counted;

    /// Return sum of contained values.
    fn sum(&self) -> Self::Kept;
    /// Return count of contained values.
    fn count(&self) -> Self::Counted;
}

/// Aggregation methods for numeric types.
pub trait NumericAggregation {
    // result which is coerced (to f64 or its container)
    type Coerced;

    /// Return mean of contained values.
    fn mean(&self) -> Self::Coerced;
    /// Return variance of contained values.
    fn var(&self) -> Self::Coerced;
    /// Return unbiased variance of contained values.
    fn unbiased_var(&self) -> Self::Coerced;
    /// Return standard deviation of contained values.
    fn std(&self) -> Self::Coerced;
    /// Return unbiased standard deviation of contained values.
    fn unbiased_std(&self) -> Self::Coerced;
}

/// Aggregation methods for comparable types.
pub trait ComparisonAggregation {
    type Kept;

    /// Return min of contained values.
    fn min(&self) -> Self::Kept;
    /// Return max of contained values.
    fn max(&self) -> Self::Kept;
}

/// Concatenate along row
pub trait Append: Sized {
    fn append(&self, other: &Self) -> Self;
}
