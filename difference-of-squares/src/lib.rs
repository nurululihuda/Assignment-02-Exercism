pub fn square_of_sum(n: u32) -> u32 {
    // unimplemented!("square of sum of 1...{}", n)
    let mut ans = 0;
    for i in 1..=n {
        ans = ans + i;
    }
    ans = ans.pow(2);
    return ans;
}

pub fn sum_of_squares(n: u32) -> u32 {
    // unimplemented!("sum of squares of 1...{}", n)
    let mut ans = 0;
    for i in 1..=n {
        ans = ans + i.pow(2);
    }
    return ans;
}

pub fn difference(n: u32) -> u32 {
    // unimplemented!(
    //     "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
    //     n = n,
    // )
    let ans;
    if square_of_sum(n) > sum_of_squares(n) {
        ans = square_of_sum(n) - sum_of_squares(n)
    } else {
        ans = sum_of_squares(n) - square_of_sum(n)
    }
    return ans;
}
