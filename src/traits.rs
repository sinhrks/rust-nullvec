

pub trait TypeDispatchScalar<T>: Sized {
    fn new(values: T) -> Self;
}

pub trait TypeDispatchVec<T>: Sized {
    fn new(values: Vec<T>) -> Self;

    fn with_mask(values: Vec<T>, mask: Option<Vec<bool>>) -> Self;
}
