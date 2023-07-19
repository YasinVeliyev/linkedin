#![allow(dead_code)]
mod insensitive_sort;
mod median;
mod print_any_text;
mod unique_items;

#[cfg(test)]
mod tests {
    use crate::insensitive_sort::sort_usernames;
    use crate::median::find_median;
    use crate::print_any_text::print_;
    use crate::unique_items::get_uniques_items;

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

    #[test]
    fn print_str() {
        let input = "Rust";
        print_(&input);
    }

    #[test]
    fn print_string() {
        let input = String::from("Rust");
        print_(&input);
    }

    #[test]
    fn five_users() {
        let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
        let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
        sort_usernames(&mut users);

        assert_eq!(users, sorted);
    }
}
