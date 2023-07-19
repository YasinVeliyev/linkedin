pub(crate) fn get_uniques_items<T: Ord + Copy>(items: Vec<T>) -> Vec<T> {
    let mut uniques_items = Vec::new();
    items.iter().for_each(|a| {
        if !uniques_items.contains(a) {
            uniques_items.push(*a)
        }
    });
    uniques_items.sort_by(|x, y| x.cmp(y));
    uniques_items
}
