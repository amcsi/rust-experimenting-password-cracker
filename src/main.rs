extern crate rayon;

use rayon::prelude::*;

fn main() {
    crack("zzzzz");
}

/// Generates a string Vec. For use with tests.
fn generate_string(i: u64) -> Vec<u8> {
    let mut array = [0u8; 20];
    generate_char_array(i, &mut array).to_vec()
}

const CHAR_COMBINATIONS: u64 = 26;

/// Generates a string slice (in u8) to crack based on an index seed.
/// An array needs to be passed to avoid performance penalties with allocating a Vec.
fn generate_char_array(mut i: u64, reversed: &mut [u8; 20]) -> &[u8] {
    if i == 0 {
        return &[];
    }
    let radix = 26;
    const A_DEC: u8 = 97;

    let mut digit = 0;
    while i > 0 {
        let remainder = i % radix;
        let remainder_zero_shifted = if remainder == 0 { radix } else { remainder };
        let m = remainder_zero_shifted - 1;

        i = (i - remainder_zero_shifted) / radix;

        reversed[digit] = A_DEC + m as u8;

        digit += 1;
    }
    for i in 0..(digit / 2) {
        let swap = reversed[digit - i - 1];
        reversed[digit - i - 1] = reversed[i];
        reversed[i] = swap;
    }
    &(reversed[0..digit])
}

fn crack(password: &str) {
    let password_bytes = Vec::from(password);

    for length in 1.. {
        // Get the seed ranges needed to check all the strings with a length of `length`.
        let range = CHAR_COMBINATIONS.pow(length - 1)..CHAR_COMBINATIONS.pow(length);
        // Now let's iterate the range in parallel; the order doesn't matter, as long as with the
        // help of the outer serial loop, we incrementally check longer and longer string length seeds.
        let option_seed = range.into_par_iter()
            .find_any(|i| {
                let mut array = [0u8; 20];
                let bytes = generate_char_array(*i, &mut array);
                return &password_bytes == &bytes;
            });
        if let Some(found_seed) = option_seed {
            println!("Found: {}", String::from_utf8(generate_string(found_seed)).unwrap());
            break;
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
    assert_eq!("", String::from_utf8(generate_string(0)).unwrap());
    assert_eq!("a", String::from_utf8(generate_string(1)).unwrap());
    assert_eq!("z", String::from_utf8(generate_string(26)).unwrap());
    assert_eq!("aa", String::from_utf8(generate_string(27)).unwrap());
    assert_eq!("zz", String::from_utf8(generate_string(702)).unwrap());
    assert_eq!("aaa", String::from_utf8(generate_string(703)).unwrap());
}
