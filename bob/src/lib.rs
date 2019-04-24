pub fn reply(message: &str) -> String {
    // unimplemented!("have Bob reply to the incoming message: {}", message)
    let v_char: Vec<char> = message.chars().collect();
    let mut ans = String::new();
    if v_char.is_empty() {
        ans.push_str("Fine. Be that way!")
    } else if (v_char[v_char.len()-2] as u32 >= 48  && v_char[v_char.len()-2] as u32 <= 57) && v_char[v_char.len()-1] == '?' {
        ans.push_str("Sure.");
    } else if v_char[v_char.len()-4] == '?' && v_char[v_char.len()-3] == ' ' && v_char[v_char.len()-2] == ' ' && v_char[v_char.len()-1] == ' ' {
        ans.push_str("Sure.");
    } else if (v_char[v_char.len()-2] as u32 >= 97  && v_char[v_char.len()-2] as u32 <= 122) && v_char[v_char.len()-1] == '!' {
        ans.push_str("Whatever.");
    } else if v_char[v_char.len()-4] as u32 >= 65 && v_char[v_char.len()-4] as u32 <= 90 && v_char[v_char.len()-3] as u32 >= 65 && v_char[v_char.len()-3] as u32 <= 90 && v_char[v_char.len()-2] as u32 >= 65 && v_char[v_char.len()-2] as u32 <= 90 && v_char[v_char.len()-1] == '?' {
        ans.push_str("Calm down, I know what I'm doing!");
    } else {
        if v_char[v_char.len()-1] == '!' {
            ans.push_str("Whoa, chill out!");
        } else if v_char[v_char.len()-1] as u32 >= 65 && v_char[v_char.len()-1] as u32 <= 90   {
            ans.push_str("Whoa, chill out!");
        } else if v_char[v_char.len()-1] == '?' {
            ans.push_str("Sure.");
        } else if v_char[0] == ' ' && v_char[v_char.len()-1] == ' ' {
            ans.push_str("Fine. Be that way!");
        } else if v_char[v_char.len()-1] == '.' {
            ans.push_str("Whatever.");
        } else if v_char[v_char.len()-1] as u32 >= 48  && v_char[v_char.len()-1] as u32 <= 57 {
            ans.push_str("Whatever.");
        } else if v_char[0] != ' ' && v_char[v_char.len()-1] == ' ' {
            ans.push_str("Whatever.");
        } else {
            ans.push_str("Fine. Be that way!");
        }
        
        
        // for j in (v_char.len() - 2)..(v_char.len()-1) {
        //     for k in (v_char.len()-1)..v_char.len() {
        //         if v_char[j] as u32 >= 65 && v_char[j] as u32 <= 90 && v_char[k] == '!' {
        //             ans.push_str("Whoa, chill out!")
        //         } else if v_char[j] as u32 >= 65 && v_char[j] as u32 <= 90 && v_char[k] as u32 >= 65 && v_char[k] as u32 <= 90 {
        //             ans.push_str("Whoa, chill out!")
        //         } else if v_char[j] as u32 >= 97 && v_char[j] as u32 <= 122 && v_char[k] == '!' {
        //             ans.push_str("Whatever.")
        //         } 
        //     }
        // }
        // for i in (v_char.len() - 1)..v_char.len() {        
        //     if v_char[i] == '.' {
        //         ans.push_str("Whatever.")
        //     } else if v_char[i] == '?' {
        //         ans.push_str("Sure.")
        //     } else if v_char[i] == ' ' {
        //         ans.push_str("Fine. Be that way!")
        //     } else if v_char[i] as u32 >= 48 && v_char[i] as u32 <= 57 {
        //         ans.push_str("Whatever.")
        //     }
    
        // }
        
    }
    ans
}
