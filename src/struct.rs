//! Structures to aid in the application

struct PasswordToCrack<'a> {
    password: &'a str,
    i: u64,
}

struct PasswordIterator {
    i: u64,
}

impl<'a> PasswordIterator {
    fn new() -> Self {
        PasswordIterator { i: 0 }
    }
}

impl Iterator for PasswordIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let i = self.i;
        self.i += 1;
        return Some(generate_string(i));
    }
}
