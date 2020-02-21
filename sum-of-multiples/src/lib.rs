use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // 思路一： 挨个判断是否能被整除
    // let mut sum = 0;
    // for x in 1..limit {
    //     let mut flag = false;
    //     for n in factors {
    //         if !flag && *n!=0 && x % n ==0 {
    //             flag = true;
    //             sum += x;
    //         }
    //     }
    // }
    // sum

    // 思路二： 构造能被整除的数
    let mut set = HashSet::new();
    for x in factors.iter() {
        if *x != 0 {
            let max = limit / x;
            for i in 1..=max {
                if i * x < limit {
                    set.insert(i * x);
                }
            }
        }
    }
    set.iter().sum()
}
