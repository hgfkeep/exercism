use std::string::String;

pub fn raindrops(n: u32) -> String {
    let mut sound: String = "".to_owned();

    if n % 3 == 0 {
        sound.push_str("Pling");
    }
    if n % 5 == 0 {
        sound.push_str("Plang");
    }
    if n % 7 == 0 {
        sound.push_str("Plong");
    }
    if sound == "" {
        sound = n.to_string();
    }
    sound
}
