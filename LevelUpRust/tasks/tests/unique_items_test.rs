use tasks::unique_items::*;

#[test]
fn unique_empty_list() {
    let input: Vec<i32> = vec![];
    let expected_output: Vec<i32> = vec![];
    let actual_output = get_uniques_items(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unique_sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = get_uniques_items(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unique_unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = get_uniques_items(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unique_unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = get_uniques_items(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unique_sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = get_uniques_items(input);
    assert_eq!(actual_output, expected_output);
}
