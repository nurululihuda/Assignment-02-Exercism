pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut string = String::new();
    for i in 0..list.len() {
        if i == list.len()-1 {
            string = string + &format!("And all for the want of a {}.", list[0]);
        } else {
            string = string + &format!("For want of a {} the {} was lost.", list[i], list[i+1]);
            string = string + &format!("\n");
        }
    }
    return string;
}
