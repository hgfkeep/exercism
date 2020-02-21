use std::cmp::Ordering;
pub fn find<T, E>(array: T, key: E) -> Option<usize>
where
    T: AsRef<[E]>,
    E: Ord,
{
    let arr = array.as_ref();
    let mut l = 0;
    let mut r = arr.len();

    while l < r {
        let mid = (l + r) / 2;
        match key.cmp(&arr[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => r = mid,
            Ordering::Greater => l = mid + 1,
        }
    }
    None
}
