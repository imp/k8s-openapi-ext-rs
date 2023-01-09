use super::*;

#[test]
fn kibi() {
    let q: Quantity = "123Ki".parse().unwrap();
    assert_eq!(q.as_int64(), Some(123 * 1024));
}

#[test]
fn mebi() {
    let q: Quantity = "123Mi".parse().unwrap();
    assert_eq!(q.as_int64(), Some(123 * 1024 * 1024));
}

#[test]
fn gibi() {
    let q: Quantity = "123Gi".parse().unwrap();
    assert_eq!(dbg!(q).as_int64(), Some(123 * 1024 * 1024 * 1024));
}

#[test]
fn tebi() {
    let q: Quantity = "123Ti".parse().unwrap();
    assert_eq!(q.as_int64(), Some(123 * 1024 * 1024 * 1024 * 1024));
}

#[test]
fn pebi() {
    let q: Quantity = "123Pi".parse().unwrap();
    assert_eq!(q.as_int64(), Some(123 * 1024 * 1024 * 1024 * 1024 * 1024));
}

#[test]
fn exbi() {
    let q: Quantity = "123Ei".parse().unwrap();
    assert_eq!(q.as_int64(), None);
    // Some(123 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024)
}

#[test]
fn kilo() {
    let q: Quantity = "123k".parse().unwrap();
    assert_eq!(q.as_int64(), Some(123_000));
}

#[test]
fn mega() {
    let q: Quantity = "28M".parse().unwrap();
    assert_eq!(q.as_int64(), Some(28_000_000));
}

#[test]
fn giga() {
    let q: Quantity = "93G".parse().unwrap();
    assert_eq!(q.as_int64(), Some(93_000_000_000));
}

#[test]
fn tera() {
    let q: Quantity = "77T".parse().unwrap();
    assert_eq!(q.as_int64(), Some(77_000_000_000_000));
}

#[test]
fn peta() {
    let q: Quantity = "40P".parse().unwrap();
    assert_eq!(q.as_int64(), Some(40_000_000_000_000_000));
}

#[test]
fn exa() {
    let q: Quantity = "19E".parse().unwrap();
    assert_eq!(q.as_int64(), None);
    // assert_eq!(q.as_int64(), Some(19_000_000_000_000_000_000));
}

#[test]
fn simple() {
    let q: Quantity = "123".parse().unwrap();
    assert_eq!(q.as_int64(), Some(123));
}

#[test]
fn zero() {
    let q: Quantity = "0".parse().unwrap();
    assert_eq!(q.as_int64(), Some(0));
}

#[test]
fn single_digit() {
    let q: Quantity = "9".parse().unwrap();
    assert_eq!(q.as_int64(), Some(9));
}

#[test]
fn double_digit() {
    let q: Quantity = "39".parse().unwrap();
    assert_eq!(q.as_int64(), Some(39));
}

#[test]
fn many_digit() {
    let q: Quantity = "27837653".parse().unwrap();
    assert_eq!(q.as_int64(), Some(27837653));
}

#[test]
fn invalid_kilo() {
    let q: Result<Quantity, _> = "253K".parse();
    assert!(q.is_err());
}
