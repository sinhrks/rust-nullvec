
macro_rules! macro_dispatch {
    ($m:ident, $( $t:ident ),* ) => {
        $(
            $m!($t);
        )*
    };
}
