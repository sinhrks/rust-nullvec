

pub trait TypeDispatchScalar<T>: Sized {
    fn new(values: T) -> Self;
}

pub trait TypeDispatch<T>: Sized {
    fn new(values: Vec<T>) -> Self;
}
