pub fn verse(n: u32) -> String {
    format!("{}{}", first_line(n), second_line(n))
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s: String = String::new();
    for x in (end..start + 1).rev() {
        s.push_str(&verse(x));
        if x != end {
            s.push_str("\n");
        }
    }
    s.to_string()
}

fn first_line(n: u32) -> String {
    let phrase = get_phrase(n);
    format!("{} on the wall, {}.\n", phrase, phrase.to_ascii_lowercase())
}

fn get_phrase(n: u32) -> String {
    let some = get_some(n);
    let qualifier = get_qualifier(n);
    format!("{} {} of beer", some, qualifier)
}

fn get_some(n: u32) -> String {
    if n == 0 {
        String::from("No more")
    } else {
        n.to_string()
    }
}

fn get_qualifier(n: u32) -> String {
    if n == 1 {
        String::from("bottle")
    } else {
        String::from("bottles")
    }
}

fn second_line(n: u32) -> String {
    let action = action(n);
    format!(
        "{}, {} on the wall.\n",
        action,
        // 为什么这里不能直接使用get_phrase(n-1),可能发生越界的问题，
        // 此处编译器会自动检查出来可能的问题！！！
        get_next_pharse(n).to_ascii_lowercase()
    )
}

fn get_next_pharse(n: u32) -> String {
    let next_n = if n == 0 { 99 } else { n - 1 };
    get_phrase(next_n)
}

fn action(n: u32) -> String {
    if n > 0 {
        let what = get_what(n);
        String::from(format!("Take {} down and pass it around", what))
    } else {
        String::from("Go to the store and buy some more")
    }
}

fn get_what(n: u32) -> String {
    if n == 1 {
        String::from("it")
    } else {
        String::from("one")
    }
}
