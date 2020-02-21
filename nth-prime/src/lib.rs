use std::f64;
pub fn nth(n: u32) -> u32 {
    match n {
        0 => return 2,
        1 => return 3,
        _ => {
            let mut count = 1_u32;
            let mut start = 5_u32;
            while count < n {
                if prime(start) {
                    count += 1;
                    if count == n {
                        break;
                    }
                }
                start += 2;
            }
            return start;
        }
    }
}

/**
 * 判断一个数是不是质数
 */
fn prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    }

    // 不在6的倍数的两侧，肯定不是质数
    if n % 6 != 1 && n % 6 != 5 {
        return false;
    }
    let tmp = (n as f64).sqrt() as u32;

    let mut i = 5;
    while i <= tmp {
        // 能被6的倍数的左右两侧一个数除尽，则不是质数
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    return true;
}
