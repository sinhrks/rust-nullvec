extern crate nullvec;
use nullvec::prelude::*;

#[test]
fn test_int_ops_add_nullable() {
    let i1 = Nullable::Value(3);
    let i2 = Nullable::Value(2);
    assert_eq!(i1 + i2, Nullable::Value(5));

    let i3 = Nullable::Value(3);
    assert_eq!(i3 + Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(Null + i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(n + Null, Null);
}

#[test]
fn test_int_ops_sub_nullable() {
    let i1 = Nullable::Value(3);
    let i2 = Nullable::Value(2);
    assert_eq!(i1 - i2, Nullable::Value(1));

    let i3 = Nullable::Value(3);
    assert_eq!(i3 - Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(Null - i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(n - Null, Null);
}

#[test]
fn test_int_ops_mul_nullable() {
    let i1 = Nullable::Value(3);
    let i2 = Nullable::Value(2);
    assert_eq!(i1 * i2, Nullable::Value(6));

    let i3 = Nullable::Value(3);
    assert_eq!(i3 * Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(Null * i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(n * Null, Null);
}

#[test]
fn test_int_ops_div_nullable() {
    let i1 = Nullable::Value(5);
    let i2 = Nullable::Value(2);
    assert_eq!(i1 / i2, Nullable::Value(2));

    let i3 = Nullable::Value(3);
    assert_eq!(i3 / Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(Null / i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(n / Null, Null);
}

#[test]
fn test_int_ops_rem_nullable() {
    let i1 = Nullable::Value(5);
    let i2 = Nullable::Value(3);
    assert_eq!(i1 % i2, Nullable::Value(2));

    let i3 = Nullable::Value(3);
    assert_eq!(i3 % Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(Null % i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(n % Null, Null);
}

#[test]
fn test_int_ops_ref_rhs_nullable() {
    let i1 = Nullable::Value(3);
    let i2 = Nullable::Value(2);
    assert_eq!(i1 + &i2, Nullable::Value(5));

    let i3 = Nullable::Value(3);
    assert_eq!(i3 + &Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(Null + &i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(n + &Null, Null);
}

#[test]
fn test_int_ops_ref_lhs_nullable() {
    let i1 = Nullable::Value(3);
    let i2 = Nullable::Value(2);
    assert_eq!(&i1 + i2, Nullable::Value(5));

    let i3 = Nullable::Value(3);
    assert_eq!(&i3 + Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(&Null + i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(&n + Null, Null);
}

#[test]
fn test_int_ops_ref_both_nullable() {
    let i1 = Nullable::Value(3);
    let i2 = Nullable::Value(2);
    assert_eq!(&i1 + &i2, Nullable::Value(5));

    let i3 = Nullable::Value(3);
    assert_eq!(&i3 + &Null, Null);
    let i4 = Nullable::Value(3);
    assert_eq!(&Null + &i4, Null);

    let n: Nullable<i64> = Null;
    assert_eq!(&n + &Null, Null);
}
