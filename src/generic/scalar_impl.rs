use std::fmt;

use super::Scalar;
use nullable::Nullable;

macro_rules! iml_scalar_as {
    ($t:ident, $as_op:ident, $is_op:ident) => {

        // ToDo: fix docstring
        impl Scalar {
            /// Convert the value to specified type
            ///
            /// The result may be undefined as the same rule as `as`
            pub fn $as_op(&self) -> Nullable<$t> {
                match self {
                    &Scalar::Null => Nullable::Null,
                    &Scalar::i64(val) => Nullable::Value(val as $t),
                    &Scalar::i32(val) => Nullable::Value(val as $t),
                    &Scalar::i16(val) => Nullable::Value(val as $t),
                    &Scalar::i8(val) => Nullable::Value(val as $t),
                    &Scalar::isize(val) => Nullable::Value(val as $t),
                    &Scalar::u64(val) => Nullable::Value(val as $t),
                    &Scalar::u32(val) => Nullable::Value(val as $t),
                    &Scalar::u16(val) => Nullable::Value(val as $t),
                    &Scalar::u8(val) => Nullable::Value(val as $t),
                    &Scalar::usize(val) => Nullable::Value(val as $t),
                    &Scalar::f64(val) => Nullable::Value(val as $t),
                    &Scalar::f32(val) => Nullable::Value(val as $t),
                    _ => panic!("cannot convert to specified type")
                }
            }

            /// Return whether the scalar is specific type
            pub fn $is_op(&self) -> bool {
                match self {
                    &Scalar::$t(_) => true,
                    _ => false
                }
            }
        }
    }
}
iml_scalar_as!(i64, as_i64, is_i64);
iml_scalar_as!(i32, as_i32, is_i32);
iml_scalar_as!(i16, as_i16, is_i16);
iml_scalar_as!(i8, as_i8, is_i8);
iml_scalar_as!(isize, as_isize, is_isize);
iml_scalar_as!(u64, as_u64, is_u64);
iml_scalar_as!(u32, as_u32, is_u32);
iml_scalar_as!(u16, as_u16, is_u16);
iml_scalar_as!(u8, as_u8, is_u8);
iml_scalar_as!(usize, as_usize, is_usize);
iml_scalar_as!(f64, as_f64, is_f64);
iml_scalar_as!(f32, as_f32, is_f32);

impl Scalar {
    pub fn as_bool(&self) -> Nullable<bool> {
        match self {
            &Scalar::Null => Nullable::Null,
            &Scalar::bool(val) => Nullable::Value(val),
            _ => panic!("cannot convert to specified type"),
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &Scalar::bool(_) => true,
            _ => false,
        }
    }
}

impl Scalar {
    pub fn as_str(&self) -> Nullable<String> {
        match self {
            &Scalar::Null => Nullable::Null,
            &Scalar::String(ref val) => Nullable::Value(val.clone()),
            _ => panic!("cannot convert to specified type"),
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            &Scalar::String(_) => true,
            _ => false,
        }
    }
}

impl Scalar {
    pub fn dtype(&self) -> String {
        match self {
            &Scalar::Null => "null".to_string(),
            &Scalar::i64(_) => "i64".to_string(),
            &Scalar::i32(_) => "i32".to_string(),
            &Scalar::i16(_) => "i16".to_string(),
            &Scalar::i8(_) => "i8".to_string(),
            &Scalar::isize(_) => "isize".to_string(),
            &Scalar::u64(_) => "u64".to_string(),
            &Scalar::u32(_) => "u32".to_string(),
            &Scalar::u16(_) => "u16".to_string(),
            &Scalar::u8(_) => "u8".to_string(),
            &Scalar::usize(_) => "usize".to_string(),
            &Scalar::f64(_) => "f64".to_string(),
            &Scalar::f32(_) => "f32".to_string(),
            &Scalar::bool(_) => "bool".to_string(),
            &Scalar::String(_) => "str".to_string(),
        }
    }
}

// Implement `Display` for `Scalar`.
impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        match self {
            &Scalar::Null => write!(f, "Null"),
            &Scalar::i64(ref val) => write!(f, "{}", val),
            &Scalar::i32(ref val) => write!(f, "{}", val),
            &Scalar::i16(ref val) => write!(f, "{}", val),
            &Scalar::i8(ref val) => write!(f, "{}", val),
            &Scalar::isize(ref val) => write!(f, "{}", val),
            &Scalar::u64(ref val) => write!(f, "{}", val),
            &Scalar::u32(ref val) => write!(f, "{}", val),
            &Scalar::u16(ref val) => write!(f, "{}", val),
            &Scalar::u8(ref val) => write!(f, "{}", val),
            &Scalar::usize(ref val) => write!(f, "{}", val),
            &Scalar::f64(ref val) => write!(f, "{}", val),
            &Scalar::f32(ref val) => write!(f, "{}", val),
            &Scalar::bool(ref val) => write!(f, "{}", val),
            &Scalar::String(ref val) => write!(f, "{}", val),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::Write;

    use super::super::Scalar;
    use nullable::Nullable;

    #[test]
    fn test_i64_dtype_property() {
        let i = Scalar::i64(1);
        assert_eq!(i.dtype(), "i64".to_string());
        assert_eq!(i.is_i64(), true);
        assert_eq!(i.is_f64(), false);
        assert_eq!(i.is_bool(), false);
        assert_eq!(i.is_str(), false);

        assert_eq!(i.as_i64(), Nullable::new(1i64));
        assert_eq!(i.as_usize(), Nullable::new(1usize));
        assert_eq!(i.as_f64(), Nullable::new(1.0f64));
    }

    #[test]
    #[should_panic]
    fn test_i64_to_bool() {
        // 1 as bool is unsupported cast
        let i = Scalar::i64(1);
        i.as_bool();
    }

    #[test]
    #[should_panic]
    fn test_i64_to_str() {
        // non numeric cast
        let i = Scalar::i64(1);
        i.as_str();
    }

    #[test]
    fn test_f64_dtype_property() {
        let f = Scalar::f64(1.1);
        assert_eq!(f.dtype(), "f64".to_string());
        assert_eq!(f.is_i64(), false);
        assert_eq!(f.is_f64(), true);
        assert_eq!(f.is_bool(), false);
        assert_eq!(f.is_str(), false);

        assert_eq!(f.as_f64(), Nullable::new(1.1f64));
        assert_eq!(f.as_i64(), Nullable::new(1.1 as i64));
        assert_eq!(f.as_usize(), Nullable::new(1.1 as usize));
    }

    #[test]
    #[should_panic]
    fn test_f64_to_bool() {
        // 1 as bool is unsupported cast
        let f = Scalar::f64(1.1);
        f.as_bool();
    }

    #[test]
    #[should_panic]
    fn test_f64_to_str() {
        // non numeric cast
        let f = Scalar::f64(1.1);
        f.as_str();
    }

    #[test]
    fn test_bool_dtype_property() {
        let b = Scalar::bool(true);
        assert_eq!(b.dtype(), "bool".to_string());
        assert_eq!(b.is_i64(), false);
        assert_eq!(b.is_f64(), false);
        assert_eq!(b.is_bool(), true);
        assert_eq!(b.is_str(), false);

        assert_eq!(b.as_bool(), Nullable::new(true));
        // assert_eq!(b.as_i64(), Nullable::new(true as i64));
    }

    #[test]
    #[should_panic]
    fn test_bool_to_f64() {
        // casting `bool` as `f64` is invalid
        let b = Scalar::bool(true);
        b.as_f64();
    }

    #[test]
    #[should_panic]
    fn test_bool_to_str() {
        // non numeric cast
        let b = Scalar::bool(true);
        b.as_str();
    }

    #[test]
    fn test_str_dtype_property() {
        let s = Scalar::String("aa".to_string());
        assert_eq!(s.dtype(), "str".to_string());
        assert_eq!(s.is_i64(), false);
        assert_eq!(s.is_f64(), false);
        assert_eq!(s.is_bool(), false);
        assert_eq!(s.is_str(), true);

        assert_eq!(s.as_str(), Nullable::new("aa".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_str_to_i64() {
        let s = Scalar::String("1".to_string());
        s.as_i64();
    }

    #[test]
    #[should_panic]
    fn test_str_to_f64() {
        let s = Scalar::String("1.1".to_string());
        s.as_f64();
    }

    #[test]
    #[should_panic]
    fn test_str_to_bool() {
        let s = Scalar::String("true".to_string());
        s.as_bool();
    }

    #[test]
    fn test_scalar_format() {
        let s = Scalar::Null;
        // better way?
        let mut buf = Vec::new();
        let _ = write!(&mut buf, "{}", s);
        assert_eq!(&buf, b"Null");

        let s = Scalar::i64(100);
        let mut buf = Vec::new();
        let _ = write!(&mut buf, "{}", s);
        assert_eq!(&buf, b"100");

        let s = Scalar::f64(1.1);
        let mut buf = Vec::new();
        let _ = write!(&mut buf, "{}", s);
        assert_eq!(&buf, b"1.1");

        let s = Scalar::bool(true);
        let mut buf = Vec::new();
        let _ = write!(&mut buf, "{}", s);
        assert_eq!(&buf, b"true");

        let s = Scalar::String("xx".to_string());
        let mut buf = Vec::new();
        let _ = write!(&mut buf, "{}", s);
        assert_eq!(&buf, b"xx");
    }
}
