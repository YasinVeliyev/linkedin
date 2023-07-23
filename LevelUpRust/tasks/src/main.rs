use tasks::validate_an_isbn_number::*;
fn main() {
    let numbers= vec![9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6,9];
    let end=9_u8;
    let numbers=vec![9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0,0];
   let a = ISBN13::check_isbn(&numbers).unwrap();
   println!("{}",a);
}
