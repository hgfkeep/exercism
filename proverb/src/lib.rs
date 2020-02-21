pub fn build_proverb(list: &[&str]) -> String {
    let mut res = String::new();
    for i in 1..list.len() {
        res.push_str(format!("For want of a {} the {} was lost.\n", list.get(i-1).unwrap(), list.get(i).unwrap()).as_str());
    }
    if list.len() > 0 {
        res.push_str(format!("And all for the want of a {}.", list.get(0).unwrap()).as_str());
    }
    res
}
