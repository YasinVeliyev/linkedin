use std::fmt::Display;

pub fn print_<T: Display>(s: &T) {
    println!("{}", s)
}
