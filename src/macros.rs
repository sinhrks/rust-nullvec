
macro_rules! macro_dispatch {
    ($m:ident, $( $t:ident ),* ) => {
        $(
            $m!($t);
        )*
    };
}


#[macro_export]
macro_rules! array {
    ($($e:expr),*) => ({
        let mut v = Vec::new();
        $(v.push($e);)*
        let nv = $crate::prelude::NullVec::new(v);
        $crate::prelude::Array::new(nv)
    });
    ($($e:expr), +, ) => (array!($($e), +))
}
