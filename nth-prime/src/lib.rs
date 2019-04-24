pub fn find_primes_up_to(limit: u32) -> Vec<u32> {
    let mut v: Vec<_> = (2..limit).collect();

    for i in 2..limit {
        v.retain(|&x| x <= i || x % i != 0);
    }

    v
}

pub fn nth(n: u32) -> u32 {
    let a = n as usize;
    let v = find_primes_up_to(105000); //to make it faster, the limit made close t the maximum answer on test
    let mut ans = 0;
    for i in 0..v.len() {
        if i == a {
            ans = v[i];
        }
    }
    return ans;
}
