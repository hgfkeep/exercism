use std::collections::HashMap;
use std::iter::FromIterator;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let map = parse_dns(dna)?;
    //⚠️考虑到即使map 无异常数据， 输入的查询参数也可能错误！！！
    match map.get(&nucleotide){
        Some(s) => Ok(*s),
        None => Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let map = parse_dns(dna)?;
    Ok(map)
}

fn parse_dns(dna: &str) -> Result<HashMap<char, usize>, char> {
    //map的默认初始化，可以直接from_iter
    let mut map: HashMap<char, usize> =
        HashMap::from_iter(vec![('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for c in dna.chars() {
        if map.contains_key(&c) {
            map.entry(c).and_modify(|x| *x += 1);
        } else {
            return Err(c);
        }
    }
    return Ok(map);
}
