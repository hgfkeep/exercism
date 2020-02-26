use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// 方法1: 回溯算法解决问题
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut char_set: HashSet<char> = HashSet::new();
    let mut first_chars: HashSet<char> = HashSet::new();
    let mut first = true;
    input.chars().for_each(|c: char| {
        if c.is_ascii_alphabetic() {
            char_set.insert(c);
            if first{
                first_chars.insert(c);
            }
            first = false;
        }else{
            first = true;
        }
    });

    let mut chars: Vec<char> = Vec::from_iter(char_set.into_iter());
    let mut map = HashMap::<char, u8>::new();
    resolve(input, &first_chars, &mut chars, &mut map)
}

/// input: 表达式
/// unset： 字母序列
/// set: 已分配数字的分配情况
/// first: 是否是第一个数
fn resolve<'a>(
    input: &'a str,
    first_chars: &'a HashSet<char>,
    unset: &'a mut Vec<char>,
    set: &'a mut HashMap<char, u8>,
) -> Option<HashMap<char, u8>> {
    //判断是否分配完成
    if let Some(c) = unset.pop() {
        //分配数字
        for x in 0..10 {

            //提前剪枝，否则字符串很长的会超过60s
            if first_chars.contains(&c) && x == 0 {
                continue;
            }
            if set.values().find(|n| &&x == n).is_none() {
                //用掉这个数字，更新状态
                set.insert(c, x);

                //回溯
                if let Some(r) = resolve(input, first_chars, unset, set) {
                    return Some(r);
                } else {
                    //状态恢复
                    set.remove(&c);
                }
            }
        }
        //状态恢复
        unset.push(c);
    } else {
        //判断结果是否符合
        let mut zero = false;
        let res: Vec<i64> = input
            .split("==")
            .map(|s| {
                s.split('+')
                    .map(|n| {
                        //转换每个加数
                        n.chars().fold(0, |r, c: char| match c {
                            c if c.is_ascii_alphabetic() => {
                                let x = set.get(&c).and_then(|x| Some(*x as i64)).unwrap();
                                if r == 0 && x == 0 {
                                    zero = true;
                                    0
                                } else {
                                    r * 10 + x
                                }
                            }
                            _ => r + 0,
                        })
                    })
                    .fold(0, |b, x| b + x) //对等式的左边或者右边求和
            })
            .collect();

        //判断左右两遍是否相等
        println!("zero={}, res={:?}, map={:?}\n", zero, res, set);
        if !zero && res.len() == 2 && res[0] == res[1] {
            return Some(set.to_owned());
        }
    }

    return None;
}
