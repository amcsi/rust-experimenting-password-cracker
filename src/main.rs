fn main() {
    crack("bb", "");
}

fn crack(password :&str, starting: &str) -> bool {
    let len = starting.len();
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
            current_string.push_str(&std::iter::repeat("a").take(depth).collect::<String>());
            a = 'a' as u8;
        } else {
            current_string.pop();
        }
        current_string.push(a as char);
        if password == &current_string {
            println!("Found: {}", current_string);
            return true;
        }
        println!("Current string: {}", current_string);
        a += 1;
    }
    false
}
