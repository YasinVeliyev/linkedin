use tasks::validate_an_isbn_number::*;

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        (vec![9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6,9], 9_u8),
        (vec![9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0,0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = ISBN13::check_isbn(case).unwrap();
        println!("{}",actual);
        assert!(actual)
    }
}

#[test]
fn rust_in_action() {
    let _:ISBN13 = "978-3-16-148410-0".parse().unwrap();
}