pub fn reply(message: &str) -> &str {
    let msg = message.trim_end();
    // end with ?   =>  Sure
    // all CAP =>      Whoa, chill out!
    // trim to "" -> Fine. Be that way!
    //other  => Whatever.
    // ALL CAP  & end with ?   => Calm down, I know what I'm doing!
    if msg.is_empty() {
        "Fine. Be that way!"
    } else {
        let end_with_question = msg.ends_with("?");

        //必须全为大写，且包含字母  =》 大写相同，小写不相同
        let all_caracter_cap = msg.ends_with(msg.to_ascii_uppercase().as_str()) && !msg.ends_with(msg.to_ascii_lowercase().as_str());

        if end_with_question && all_caracter_cap {
            "Calm down, I know what I'm doing!"
        } else if end_with_question {
            "Sure."
        } else if all_caracter_cap {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    }
}
