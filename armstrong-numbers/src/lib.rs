fn split_int(n: usize) -> Vec<usize> {
    fn split(n: usize, v: &mut Vec<usize>) {
        if n >= 10 {
            split(n / 10, v);
        }
        v.push(n % 10);
    }
    let mut v = Vec::new();
    split(n, &mut v);
    v
}

pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let a = *&num as usize;
    let s = split_int(a);
    let mut ans = 0;
    let p = s.len() as u32;
    for i in 0..s.len() {
        ans = ans + s[i].pow(p);
    }   
    if ans == a {
        true
    } else {
        false
    }
}
