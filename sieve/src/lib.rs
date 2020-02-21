pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // let mut res: Vec<u64> = Vec::new();
    let mut is_primes = vec![true; upper_bound as usize + 1];

    // for i in 2..=upper_bound {
    //     if is_primes[i as usize] {
    //         res.push(i);

    //         let mut j = i + i;
    //         while j <= upper_bound {
    //             is_primes[j as usize] = false;

    //             j += i;
    //         }
    //     }
    // }

    (2..=upper_bound).filter_map(|i| {
        //对于每个元素，如果不是素数，返回None，过滤掉
        if !is_primes[i as usize] {
            None
        }else{
            (2..).map(|j| i*j)// 对于i的n倍（n>=2）的所有的数，都不是素数
            .take_while(|x| *x<=upper_bound)
            .for_each(|x | is_primes[x as usize] = false);
            Some(i)
        }
    }).collect()

    // res
}
