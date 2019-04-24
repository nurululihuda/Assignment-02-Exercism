pub fn series(digits: &str, len: usize) -> Vec<String> {
    // unimplemented!(
    //     "What are the series of length {} in string {:?}",
    //     len,
    //     digits
    // )
    let mut ans = Vec::new();
    if digits.len() < len {
       return ans;
    }
    for i in 0..=(digits.len()-len) {
        ans.push((digits[i..(len+i)]).to_string())
    }
    ans
    
}