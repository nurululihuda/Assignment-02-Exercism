pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    let mut v = vec!(0);
    let mut ans = 0;
    for i in 1..limit {
        for j in 0..factors.len() {
            if factors[j] != 0 {
                if i % factors[j] == 0 {
                v.push(i)
                }
            }                              
        }
    }
    v.dedup();
    for k in 0..v.len(){
        ans = ans + v[k];
    }
    return ans;
}