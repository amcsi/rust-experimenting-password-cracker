fn main() {
    crack("zzzzz", &Vec::new());
}

fn crack(password :&str, starting: &Vec<u8>) -> bool {
    let password_bytes = Vec::from(password);
    let mut current_string:Vec<u8> = starting.clone();

    loop {
        let mut a = 'a' as u8;
        while a <= ('z' as u8) {
            current_string.push(a);
            if &password_bytes == &current_string {
                match String::from_utf8(current_string) {
                    Ok(found_string) => println!("Found: {}", found_string),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                }
                return true;
            }
//            println!("Current: {}", current_string);
            current_string.pop();
            a += 1;
        }

        let mut depth = 1;
        while let Some(last_char) = current_string.pop() {
            if last_char == 'z' as u8  {
                depth += 1;
                continue;
            } else {
                current_string.push((last_char as u8) + 1);
                depth -= 1;
                break;
            }
        }
        while depth > 0 {
            current_string.push('a' as u8);
            depth -= 1;
        }
    }
}
