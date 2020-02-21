pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];

    let mut f = 2_u64;
    let mut remain = n;
    while f <= remain {
        if remain % f ==0 {
            // 此处f是mut的，无法borrow是， 必须克隆内存
            res.push(f);
            remain /= f;
            f = 2_u64;
        }else{
            f += 1;
        }
    }

    res
}