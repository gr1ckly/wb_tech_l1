use std::any::{type_name};

pub fn check_type<T>(value: &T){
    println!("{}", type_name::<T>());
}