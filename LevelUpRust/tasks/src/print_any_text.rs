use std::fmt::Display;

pub(crate) fn print_<T: Display>(s: &T) {
    println!("{}", s)
}
