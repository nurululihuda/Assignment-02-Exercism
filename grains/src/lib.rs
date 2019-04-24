pub fn square(s: u32) -> u64 {
    // unimplemented!("grains of rice on square {}", s);
    if s >= 1 && s <= 64 {
        2u64.pow(s-1)
    } else {
        panic!("Square must be between 1 and 64")
    }
    
}

pub fn total() -> u64 {
    let s = 1u32;
    let mut ans = 0;
    for s in s..65 {
        ans = ans + 2u64.pow(s-1)
    }
    return ans;
}
