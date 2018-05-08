extern crate rayon;

use rayon::prelude::*;
use std::str;

fn main() {
    crack("zzzzz", &Vec::new());
}

struct PasswordToCrack<'a> {
    password: &'a str,
    i: u64,
}

/// Generates a string to crack based on an index seed
fn generate_string(mut i: u64) -> Vec<u8> {
    let mut result = vec![];
    if i == 0 {
        return result;
    }
    let radix = 26;
    const A_DEC: u8 = 97;

    while i > 0 {
        let remainder = i % radix;
        let remainder_zero_shifted = if remainder == 0 { radix } else { remainder };
        let m = remainder_zero_shifted - 1;

        i = (i - remainder_zero_shifted) / radix;

        result.push(A_DEC + m as u8);
    }
    result.into_iter().rev().collect()
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

#[test]
fn test_calculate_str_len() {
    assert_eq!(0, generate_string(0).len());
    assert_eq!(1, generate_string(1).len());
    assert_eq!(1, generate_string(26).len());
    assert_eq!(2, generate_string(27).len());
    assert_eq!(2, generate_string(702).len());
    assert_eq!(3, generate_string(703).len());
}


#[test]
fn test_generate_string() {
    assert_eq!("", str::from_utf8(&generate_string(0)[..]).unwrap());
    assert_eq!("a", str::from_utf8(&generate_string(1)[..]).unwrap());
    assert_eq!("z", str::from_utf8(&generate_string(26)[..]).unwrap());
    assert_eq!("aa", str::from_utf8(&generate_string(27)[..]).unwrap());
    assert_eq!("zz", str::from_utf8(&generate_string(702)[..]).unwrap());
    assert_eq!("aaa", str::from_utf8(&generate_string(703)[..]).unwrap());
}
