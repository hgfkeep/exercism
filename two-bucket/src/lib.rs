#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    if reachable(capacity_2, capacity_1, goal) {
        let buckets: [u8; 2] = [capacity_1, capacity_2];

        //开始倒水的桶index
        let mut start_index: usize = 0;
        match start_bucket {
            Bucket::Two => start_index = 1,
            _ => start_index = 0,
        }
        let other_index = (start_index + 1) %2;
        //中间状态
        let mut state: [u8; 2] = [0, 0];

        let mut counter: u8 = 0;

        //不是goal的状态，则循环处理
        while state[other_index]!= goal && state[start_index]!=goal {
            //开始桶没水了，倒满
            if state[start_index] == 0 {
                state[start_index] = buckets[start_index];
                counter +=1;
                println!("({}, {})\t {} full",state[0], state[1], start_index );
            }else if state[start_index] + state[other_index] < buckets[other_index] {
                //两个桶剩余量小于 other 桶的量，则全部倒入other桶的量
                state[other_index] = state[start_index] + state[other_index];
                state[start_index] = 0;
                counter +=1;
                println!("({}, {})\tpour {} into {}",state[0], state[1], start_index, other_index );
            }else if state[other_index] == buckets[other_index] ||
            // n * buckets[start_index] = buckets[other_index]时，可能出现如下情况
             state[start_index] + state[other_index] == buckets[other_index] {
                // other 桶满了，则需要倒掉
                state[other_index] = 0;
                counter +=1;
                println!("({}, {})\tempty {}",state[0], state[1], other_index );
            }else{
                //other 桶未满，且start 桶有水，则直接倒倒other桶。
                let total_tmp = state[start_index] + state[other_index];
                state[other_index] = buckets[other_index];
                state[start_index] = total_tmp - buckets[other_index];
                println!("({}, {})\tpour {} into {}",state[0], state[1], start_index, other_index );
                counter +=1;
            }
            println!("({}, {})\n\n", state[0], state[1]);
        }
        let buckets_ref = [Bucket::One, Bucket::Two];
        
        if state[start_index] == goal {
            match start_index {
                0 => return Some(BucketStats{moves: counter, goal_bucket: Bucket::One, other_bucket:state[other_index]}),
                _ =>return Some(BucketStats{moves: counter, goal_bucket: Bucket::Two, other_bucket:state[other_index]}),
            }
            
        }else if state[other_index] == goal {
            match other_index {
                0 => return Some(BucketStats{moves: counter, goal_bucket: Bucket::One, other_bucket:state[start_index]}),
                _ =>return Some(BucketStats{moves: counter, goal_bucket: Bucket::Two, other_bucket:state[start_index]}),
            }
        }
    }

    None
}

///am + bn = x 有解，当前仅x可以被a和b的最大公约数 `gcd(a,b)` 整除， 即 `x % gcd(a,b) = 0`
pub fn reachable(a: u8, b: u8, goal: u8) -> bool {
    let gcd = gcd(a, b);
    return goal % gcd == 0;
}

pub fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
