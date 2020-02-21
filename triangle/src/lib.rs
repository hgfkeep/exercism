use std::cmp::Ord;
use std::collections::BTreeSet;
use std::ops::Add;

pub struct Triangle<T: Add> {
    len_set: BTreeSet<T>,
}

impl<T> Triangle<T>
where
    //注意Add trait 类型参数的写法， 注意同时满足多个trait约束的写法
    T: Add<Output = T> + Ord + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let pair: [(T, T); 3] = [
            (sides[0] + sides[1], sides[2]),
            (sides[0] + sides[2], sides[1]),
            (sides[2] + sides[1], sides[0]),
        ];
        //sides.iter().all(|&len| len > 0) 不能校验每条边必须大于0
        if pair.iter().all(|(l, r)| l > r) {
            return Some(Triangle {
                len_set: sides.iter().map(|&x| x).collect(),
            });
        }
        None
    }

    pub fn is_equilateral(&self) -> bool {
        self.len_set.len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.len_set.len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.len_set.len() == 2
    }
}
