pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {}", n)
    let mut v: Vec<u64> = vec![];
    let mut m = n;
    let mut c = 2;
    let mut done = false;
    while !done {
        if m >=2 {
            while m % c == 0 {
                v.push(c);
                m = m / c;                                        
            }
            c += 1;
        } else {
            done = true;
        }
    }
    return v;
}
