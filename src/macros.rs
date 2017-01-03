
macro_rules! dispatch {
    ($m:ident) => {
        $m!(i64);
        $m!(i32);
        $m!(i16);
        $m!(i8);
        $m!(isize);
        $m!(u64);
        $m!(u32);
        $m!(u16);
        $m!(u8);
        $m!(usize);
        $m!(bool);
        $m!(String);
    }
}
