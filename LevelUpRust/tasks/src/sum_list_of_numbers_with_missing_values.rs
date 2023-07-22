pub fn sum_with_missing(list: Vec<Option<i32>>) -> i32 {
    list.into_iter().map(|x| x.unwrap_or(0)).sum()
}
