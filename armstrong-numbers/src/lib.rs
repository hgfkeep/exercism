pub fn is_armstrong_number(num: u32) -> bool {
    let mut coll: Vec<u32> = vec![];
    let mut n = num;
    while n > 0 {
        coll.push(n % 10);
        n /= 10;
    }
    let l = coll.len();
    for x in coll.iter() {
        n += x.pow(l as u32);
    }

    return n == num;
}
