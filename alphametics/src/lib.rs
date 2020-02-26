use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

//方法2:回溯算法解决问题
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    //所有字符集合
    let mut char_set: HashSet<char> = HashSet::new();
    //所有第一个字符
    let mut first_chars: HashSet<char> = HashSet::new();
    //所有数列表
    let mut nums: Vec<String> = Vec::new();

    //临时变量
    let mut first = true;
    let mut num: String = String::new();
    input.chars().for_each(|c: char| {
        if c.is_ascii_alphabetic() {
            num.push(c);
            char_set.insert(c);
            if first {
                first_chars.insert(c);
            }
            first = false;
        } else {
            first = true;
            if num.len() > 0 {
                nums.push(num.clone());
                num.clear();
            }
        }
    });

    //⚠️ 缺少插入最后一个数字
    if num.len() > 0 {
        nums.push(num);
    }
    let mut chars: Vec<&char> = Vec::from_iter(char_set.iter());
    let mut map = HashMap::<char, u8>::new();
    resolve(&nums, &mut chars, &first_chars, &mut map)
}

///
/// nums：解析出来的所有数，其中最后一个数是等号后面的数
/// char_set： 所有字符的分布
/// first_chats: 数字的第一个字母
/// set： 回溯法尝试的字母数字对应关系
fn resolve<'a>(
    nums: &'a Vec<String>,
    char_set: &'a mut Vec<&char>,
    first_chars: &'a HashSet<char>,
    set: &'a mut HashMap<char, u8>,
) -> Option<HashMap<char, u8>> {
    if let Some(c) = char_set.pop() {
        for x in 0..10 {
            //数字开始为0, 或 数字重复出现, 则跳过
            if !((first_chars.contains(&c) && x == 0) || set.values().any(|v| *v == x)) {
                //插入尝试的情况, 两数字不能相等
                set.insert(*c, x);

                //回溯和判断回溯是否成功
                if let Some(ret) = resolve(nums, char_set, first_chars, set) {
                    return Some(ret);
                } else {
                    //回溯不成功，恢复现场
                    set.remove(&c);
                }
            }
        }

        char_set.push(c);
    } else {
        //判断是否相等
        let mut res: Vec<i64> = nums
            .iter()
            .map(|num| {
                num.chars().fold(0, |n: i64, c| {
                    n * 10 + set.get(&c).and_then(|s| Some(*s as i64)).unwrap()
                })
            })
            .collect();

        if let Some(last) = res.pop() {
            if last == res.iter().sum() {
                return Some(set.to_owned());
            }
        }
    }

    None
}
