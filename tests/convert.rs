extern crate nullvec;
use nullvec::prelude::*;

#[test]
#[should_panic]
fn test_empty_scalar_to_array() {
    let vals: Vec<Scalar> = vec![];
    let _: Array = vals.into();
}

#[test]
fn test_i64_vec_to_array() {
    let exp: Array = Array::Int64Array(NullVec::new(vec![1, 2]));

    // Into
    let vals: Vec<i64> = vec![1, 2];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    // From
    let vals: Vec<i64> = vec![1, 2];
    let res = Array::from(vals);
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];
    let res = Array::from(vals);
    assert_eq!(res, exp);
}

#[test]
fn test_i64_array_to_vec() {
    let exp: Vec<i64> = vec![1, 2];
    let exps: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];

    // Into
    let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
    let res: Vec<i64> = vals.into();
    assert_eq!(res, exp);

    let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
    let res: Vec<Scalar> = vals.into();
    assert_eq!(res, exps);

    // From
    let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
    let res: Vec<i64> = Vec::from(vals);
    assert_eq!(res, exp);

    let vals = Array::Int64Array(NullVec::new(vec![1, 2]));
    let res: Vec<Scalar> = Vec::from(vals);
    assert_eq!(res, exps);
}

#[test]
fn test_usize_vec_to_array() {
    let exp: Array = Array::UsizeArray(NullVec::new(vec![1, 2]));

    // Into
    let vals: Vec<usize> = vec![1, 2];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::usize(1), Scalar::usize(2)];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    // From
    let vals: Vec<usize> = vec![1, 2];
    let res = Array::from(vals);
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::usize(1), Scalar::usize(2)];
    let res = Array::from(vals);
    assert_eq!(res, exp);
}

#[test]
fn test_usize_array_to_vec() {
    let exp: Vec<usize> = vec![1, 2];
    let exps: Vec<Scalar> = vec![Scalar::usize(1), Scalar::usize(2)];

    // Into
    let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
    let res: Vec<usize> = vals.into();
    assert_eq!(res, exp);

    let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
    let res: Vec<Scalar> = vals.into();
    assert_eq!(res, exps);

    // From
    let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
    let res: Vec<usize> = Vec::from(vals);
    assert_eq!(res, exp);

    let vals = Array::UsizeArray(NullVec::new(vec![1, 2]));
    let res: Vec<Scalar> = Vec::from(vals);
    assert_eq!(res, exps);
}

#[test]
fn test_f64_vec_to_array() {
    let exp: Array = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));

    // Into
    let vals: Vec<f64> = vec![1.1, 2.2];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    // From
    let vals: Vec<f64> = vec![1.1, 2.2];
    let res = Array::from(vals);
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];
    let res = Array::from(vals);
    assert_eq!(res, exp);
}

#[test]
fn test_f64_array_to_vec() {
    let exp: Vec<f64> = vec![1.1, 2.2];
    let exps: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];

    // Into
    let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
    let res: Vec<f64> = vals.into();
    assert_eq!(res, exp);

    let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
    let res: Vec<Scalar> = vals.into();
    assert_eq!(res, exps);

    // From
    let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
    let res: Vec<f64> = Vec::from(vals);
    assert_eq!(res, exp);

    let vals = Array::Float64Array(NullVec::new(vec![1.1, 2.2]));
    let res: Vec<Scalar> = Vec::from(vals);
    assert_eq!(res, exps);
}

#[test]
fn test_bool_vec_to_array() {
    let exp: Array = Array::BoolArray(NullVec::new(vec![true, false]));

    // Into
    let vals: Vec<bool> = vec![true, false];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    // From
    let vals: Vec<bool> = vec![true, false];
    let res = Array::from(vals);
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];
    let res = Array::from(vals);
    assert_eq!(res, exp);
}

#[test]
fn test_bool_array_to_vec() {
    let exp: Vec<bool> = vec![true, false];
    let exps: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];

    // Into
    let vals = Array::BoolArray(NullVec::new(vec![true, false]));
    let res: Vec<bool> = vals.into();
    assert_eq!(res, exp);

    let vals = Array::BoolArray(NullVec::new(vec![true, false]));
    let res: Vec<Scalar> = vals.into();
    assert_eq!(res, exps);

    // From
    let vals = Array::BoolArray(NullVec::new(vec![true, false]));
    let res: Vec<bool> = Vec::from(vals);
    assert_eq!(res, exp);

    let vals = Array::BoolArray(NullVec::new(vec![true, false]));
    let res: Vec<Scalar> = Vec::from(vals);
    assert_eq!(res, exps);
}

#[test]
fn test_str_vec_to_array() {
    let exp: Array = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));

    // Into
    let vals: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    let vals: Vec<&str> = vec!["a", "b"];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![
        Scalar::String("a".to_string()),
        Scalar::String("b".to_string()),
    ];
    let res: Array = vals.into();
    assert_eq!(res, exp);

    // From
    let vals: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let res = Array::from(vals);
    assert_eq!(res, exp);

    let vals: Vec<&str> = vec!["a", "b"];
    let res = Array::from(vals);
    assert_eq!(res, exp);

    let vals: Vec<Scalar> = vec![
        Scalar::String("a".to_string()),
        Scalar::String("b".to_string()),
    ];
    let res = Array::from(vals);
    assert_eq!(res, exp);
}

#[test]
fn test_str_array_to_vec() {
    let exp: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let exps: Vec<Scalar> = vec![
        Scalar::String("a".to_string()),
        Scalar::String("b".to_string()),
    ];

    // Into
    let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
    let res: Vec<String> = vals.into();
    assert_eq!(res, exp);

    let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
    let res: Vec<Scalar> = vals.into();
    assert_eq!(res, exps);

    // From
    let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
    let res: Vec<String> = Vec::from(vals);
    assert_eq!(res, exp);

    let vals = Array::StringArray(NullVec::new(vec!["a".to_string(), "b".to_string()]));
    let res: Vec<Scalar> = Vec::from(vals);
    assert_eq!(res, exps);
}

#[test]
fn test_i64_primitives_to_scalar() {
    let exp = Scalar::i64(1);

    let res: Scalar = 1i64.into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from(1i64);
    assert_eq!(res, exp);
}

#[test]
fn test_i64_scalar_to_primitives() {
    let res: i64 = Scalar::i64(1).into();
    assert_eq!(res, 1);

    let res: i64 = i64::from(Scalar::i64(1));
    assert_eq!(res, 1);
}

#[test]
fn test_f64_primitives_to_scalar() {
    let exp = Scalar::f64(1.1);

    let res: Scalar = (1.1).into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from(1.1);
    assert_eq!(res, exp);
}

#[test]
fn test_f64_scalar_to_primitives() {
    let res: f64 = Scalar::f64(1.1).into();
    assert_eq!(res, 1.1);

    let res: f64 = f64::from(Scalar::f64(1.1));
    assert_eq!(res, 1.1);
}

#[test]
fn test_bool_primitives_to_scalar() {
    let exp = Scalar::bool(true);

    let res: Scalar = true.into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from(true);
    assert_eq!(res, exp);
}

#[test]
fn test_bool_scalar_to_primitives() {
    let res: bool = Scalar::bool(true).into();
    assert_eq!(res, true);

    let res: bool = bool::from(Scalar::bool(true));
    assert_eq!(res, true);
}

#[test]
fn test_str_primitives_to_scalar() {
    let exp = Scalar::String("a".to_string());

    let res: Scalar = "a".to_string().into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from("a".to_string());
    assert_eq!(res, exp);

    // &str
    let res: Scalar = "a".into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from("a");
    assert_eq!(res, exp);
}


#[test]
fn test_str_scalar_to_primitives() {
    let res: String = Scalar::String("a".to_string()).into();
    assert_eq!(res, "a".to_string());

    let res: String = String::from(Scalar::String("a".to_string()));
    assert_eq!(res, "a".to_string());
}

// impl<'a> From<&'a str> for Scalar {
//     fn from(value: &'a str) -> Self {
//         if value == "Null" {
//             Scalar::Null
//         } else {
//             value.parse().map(Scalar::i64)
//                 .or_else(|_| value.parse().map(Scalar::f64)
//                          .or_else(|_| value.parse().map(Scalar::bool)))
//                 .unwrap_or_else(|_| Scalar::String(value.to_string()))
//         }
//     }
// }

// impl From<Scalar> for String {
//     fn from(value: Scalar) -> Self {
//         match value {
//             Scalar::i64(val) => val.to_string(),
//             Scalar::f64(val) => val.to_string(),
//             Scalar::bool(val) => val.to_string(),
//             Scalar::String(val) => val,
//             _ => panic!("Unable to convert to primitive")
//         }
//     }
// }

#[test]
fn test_scalar_to_primitives_as_string() {
    // Null
    let exp = "Null";
    let res: String = Scalar::Null.into();
    assert_eq!(res, exp);

    let res: String = String::from(Scalar::Null);
    assert_eq!(res, exp);

    // f64
    let exp = "0.1";
    let res: String = Scalar::f64(0.1).into();
    assert_eq!(res, exp);

    let res: String = String::from(Scalar::f64(0.1));
    assert_eq!(res, exp);

    // i64
    let exp = "10";
    let res: String = Scalar::i64(10).into();
    assert_eq!(res, exp);

    let res: String = String::from(Scalar::i64(10));
    assert_eq!(res, exp);

    // bool
    let exp = "true";
    let res: String = Scalar::bool(true).into();
    assert_eq!(res, exp);

    let res: String = String::from(Scalar::bool(true));
    assert_eq!(res, exp);

    // String
    let exp = "Hello world";
    let res: String = Scalar::String("Hello world".to_string()).into();
    assert_eq!(res, exp);

    let res: String = String::from(Scalar::String("Hello world".to_string()));
    assert_eq!(res, exp);
}

#[test]
fn test_primitives_as_str_to_scalar() {
    // Null
    let exp = Scalar::Null;
    let res: Scalar = "Null".into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from("Null");
    assert_eq!(res, exp);

    // f64
    let exp = Scalar::f64(0.1);
    let res: Scalar = "0.1".into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from("0.1");
    assert_eq!(res, exp);

    // i64
    let exp = Scalar::i64(10);
    let res: Scalar = "10".into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from("10");
    assert_eq!(res, exp);

    // bool
    let exp = Scalar::bool(true);
    let res: Scalar = "true".into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from("true");
    assert_eq!(res, exp);

    // String
    let exp = Scalar::String("Hello world".to_string());
    let res: Scalar = "Hello world".into();
    assert_eq!(res, exp);

    let res: Scalar = Scalar::from("Hello world");
    assert_eq!(res, exp);
}
