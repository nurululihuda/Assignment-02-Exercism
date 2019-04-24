pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)
    if n % 3 == 0 && n % 5 != 0 && n % 7 != 0 {
        format!("{}", "Pling".to_string())
    } else if n % 5 == 0 && n % 3 != 0 && n % 7 != 0 {
        format!("{}", "Plang".to_string())
    } else if n % 7 == 0 && n % 3 != 0 && n % 5 != 0 {
        format!("{}", "Plong".to_string())
    } else if n % 3 == 0 && n % 5 == 0 && n % 7 != 0{
        format!("{}", "PlingPlang".to_string())
    } else if n % 3 == 0 && n % 7 == 0 && n % 5 != 0{
        format!("{}", "PlingPlong".to_string())
    } else if n % 5 == 0 && n % 7 == 0 && n % 3 != 0{
        format!("{}", "PlangPlong".to_string())
    } else if n % 3 == 0 && n % 5 == 0 && n % 7 == 0 {
        format!("{}", "PlingPlangPlong".to_string())
    } else {
        format!("{}", n)
    }
}
