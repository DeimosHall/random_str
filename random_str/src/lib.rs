use rand::prelude::*;

/// Get a random char from a-z or A-Z
///
/// # Examples
///
/// ```
/// let random_char = random_str::get_random_char(false);
/// println!("Random char: {}", random_char);
/// ```
/// Possible output: Random char: w
/// ```
/// let random_char = random_str::get_random_char(true);
/// println!("Random char: {}", random_char);
/// ```
/// Possible output: Random char: X
pub fn get_random_char(capital_letter: bool) -> char {
    let mut chars: Vec<char> = (b'a'..b'z').map(|c| c as char).collect();
    
    if capital_letter {
        let capital_chars: Vec<char> = (b'A'..b'Z').map(|c| c as char).collect();
        chars.extend(capital_chars);
    }

    let mut rng = thread_rng();
    let random_char = chars.choose(&mut rng).unwrap();
    *random_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_char() {
        let random_char = get_random_char(false);
        assert!(random_char.is_ascii_alphabetic());
        assert!(random_char.is_ascii_lowercase());

        for _ in 0..100 {
            let random_char = get_random_char(true);
            if random_char.is_ascii_uppercase() {
                assert!(true);
                return;
            }
        }
    }
}
