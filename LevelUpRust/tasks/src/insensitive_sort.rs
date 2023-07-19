pub fn sort_usernames(usernames: &mut Vec<&str>) {
    usernames.sort_by(|x, y| x.to_lowercase().cmp(&y.to_lowercase()));
}
