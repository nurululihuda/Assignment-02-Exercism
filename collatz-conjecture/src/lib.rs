pub fn collatz(n: u64) -> Option<u64> {
    // unimplemented!(
    //     "return Some(x) where x is the number of steps required to reach 1 starting with {}",
    //     n,
    // )
    let mut x = 0;
    let mut a = n; 
    while a > 1 {
        if a % 2 == 0 {
            a = a/2;
            x += 1;
        } else {
            a = a*3+1;
            x += 1;
        }
    } 
    if a == 1 {
        Some(x)
    } else {
        None
    }
    
}
