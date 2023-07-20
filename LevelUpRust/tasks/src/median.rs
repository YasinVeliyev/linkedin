pub fn find_median(mut v: Vec<f32>) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let middle = v.len() / 2;

    if v.len() % 2 == 1 {
        return Some(v[middle]);
    }

    Some((v[middle - 1] + v[middle]) / 2.0)
}
