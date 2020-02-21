pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .flat_map(|word| {
            // 取出整句第一个字母 + 与第一个字母不连续大写的大写字母
            word.chars()
                .filter(|c: &char| c.is_ascii_alphabetic())
                //取出一个后不会继续执行filter
                .take(1)
                .chain(
                    word.chars()
                    //过滤出字符
                        .filter(|c| c.is_alphabetic())
                        //跳过开始的连续大写字符
                        .skip_while(|c| c.is_uppercase())
                        //选出除开始的连续的大写字母的后续大写字母
                        .filter(|c| c.is_uppercase()),
                )
        })
        .collect::<String>()
        .to_uppercase()
}
