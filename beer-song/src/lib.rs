pub fn verse(n: i32) -> String {
    // unimplemented!("emit verse {}", n)
    if n == 0 {
        format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else if n == 1 {
        format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    } else if n == 2 {
        format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
    } else {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
    }

}

pub fn sing(start: i32, end: i32) -> String {
    // unimplemented!("sing verses {} to {}, inclusive", start, end)
    let mut song = String::new();
    for n in (end..=start).rev() {
        if n > end {
            song.push_str(&(verse(n) + "\n"));
        } else {
            song.push_str(&verse(n));
        }
    }
    return song;
}
