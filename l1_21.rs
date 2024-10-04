use std::str::FromStr;
use num::BigInt;

pub fn sum(a: &str, b: &str) -> String{
    let a = BigInt::from_str(a).unwrap();
    let b = BigInt::from_str(b).unwrap();
    (a + b).to_string()
}
pub fn difference(a: &str, b: &str) -> String{
    let a = BigInt::from_str(a).unwrap();
    let b = BigInt::from_str(b).unwrap();
    (a - b).to_string()
}
pub fn multiplication(a: &str, b: &str) -> String{
    let a = BigInt::from_str(a).unwrap();
    let b = BigInt::from_str(b).unwrap();
    (a * b).to_string()
}

pub fn divide(a: &str, b: &str) -> String{
    let a = BigInt::from_str(a).unwrap();
    let b = BigInt::from_str(b).unwrap();
    (a / b).to_string()
}