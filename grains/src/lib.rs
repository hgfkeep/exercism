pub fn square(s: u32) -> u64 {
    assert!(s >=1 && s <=64, "Square must be between 1 and 64");
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    //2_u64.pow(64)-1 但是2_u64.pow(64）会越界
    2_u64.pow(63)-1+2_u64.pow(63)
}
