fn main() {
    crack("zzzzz", "");
}

fn crack(password :&str, starting: &str) -> bool {
    let mut current_string = starting.to_string();

    let mut a = 'z' as u8 + 1;
    loop {
        if a > 'z' as u8 {
            let mut depth = 0;
            while let Some(last_char) = current_string.pop() {
                if last_char == 'z' {
                    depth += 1;
                    continue;
                } else {
                    current_string.push(((last_char as u8) + 1) as char);
                    depth -= 1;
                    break;
                }
            }
            while depth > 0 {
                current_string.push('a');
                depth -= 1;
            }
            a = 'a' as u8;
        } else {
            current_string.pop();
        }
        current_string.push(a as char);
        if password == &current_string {
            println!("Found: {}", current_string);
            return true;
        }
//        println!("Current: {}", current_string);
        a += 1;
    }
}
