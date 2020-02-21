pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mut l: usize = 0;
    let mut r: usize = array.len() - 1; //usize<0会panic
    while l <= r {
        let mid = (l + r) / 2;
        if key == array[mid] {
            return Some(mid);
        } else if key < array[mid] && mid > 0 {
            //usize < 0会panic
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    None
}
