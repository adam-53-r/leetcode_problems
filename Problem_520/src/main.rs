pub fn detect_capital_use(word: String) -> bool {

    let mut chars = word.chars();

    let len = chars.size_hint().1.unwrap();

    if len == 1 {return true;}

    let mut correct_capital_use = true;

    if chars.next().unwrap().is_lowercase() {
        
        for _char in chars {
            if _char.is_uppercase() {
                correct_capital_use = false;
            }
        }

    }
    else {
        
        if chars.next().unwrap().is_lowercase() {
        
            for _char in chars {
                if _char.is_uppercase() {
                    correct_capital_use = false;
                }
            }
    
        }
        else {
            for _char in chars {
                if _char.is_lowercase() {
                    correct_capital_use = false;
                }
            }
        }

    }
    
    return correct_capital_use;
}


fn main() {
    println!("{}", detect_capital_use(String::from("UWU")));
    println!("{}", detect_capital_use(String::from("U")));
}
