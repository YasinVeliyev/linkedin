use tasks::median::*;

#[test]
fn test_median_odd_length() {
    let numbers = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    assert_eq!(find_median(numbers), Some(5.0));
}

#[test]
fn test_median_even_length() {
    let numbers = vec![2., 4., 6., 8.];
    assert_eq!(find_median(numbers), Some(5.0));
}

#[test]
fn test_median_empty_list() {
    let numbers: Vec<f32> = vec![];
    assert_eq!(find_median(numbers), None);
}

#[test]
fn test_median_single_element() {
    let numbers = vec![42.];
    assert_eq!(find_median(numbers), Some(42.0));
}
