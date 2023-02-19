use rand::prelude::*;

/// Get a random char from a-z or A-Z
/// 
/// Definition of the function:
/// ```
/// pub fn get_random_letter(lower_letter: bool, capital_letter: bool) -> char
/// ```
/// 
/// # Panics
/// Panics if the function is called with both parameters set to false.
///
/// # Examples
///
/// ```
/// use random_str as random;
/// ```
/// 
/// ```
/// let random_letter = random::get_random_letter(true, false);
/// println!("Random letter: {}", random_char);
/// ```
/// Possible output: Random letter: w
/// ```
/// let random_letter = random::get_random_letter(false, true);
/// println!("Random letter: {}", random_letter);
/// ```
/// Possible output: Random letter: X
/// ```
/// let random_letter = random::get_random_letter(true, true);
/// println!("Random letter: {}", random_letter);
/// ```
/// Possible output: Random letter: x or Random letter: Y
pub fn get_random_letter(lower_letter: bool, capital_letter: bool) -> char {
    let mut chars: Vec<char> = vec![];

    if lower_letter {
        let lower_chars: Vec<char> = (b'a'..b'z').map(|c| c as char).collect();
        chars = lower_chars;
    }
    
    if capital_letter {
        let capital_chars: Vec<char> = (b'A'..b'Z').map(|c| c as char).collect();
        chars.extend(capital_chars);
    }

    let mut rng = thread_rng();
    let random_char = chars.choose(&mut rng).unwrap();
    *random_char
}

pub fn get_random_int(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn get_random_symbol() -> char {
    let symbols: Vec<char> = vec!['#', '$', '%', '&', '*', '@', '^'];
    let mut rng = thread_rng();
    let random_symbol = symbols.choose(&mut rng).unwrap();
    *random_symbol
}

pub fn get_random_string(length: usize, lower_letter: bool, capital_letter: bool) -> String {
    let mut random_string = String::new();
    for _ in 0..length {
        random_string.push(get_random_letter(lower_letter, capital_letter));
    }
    random_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_char() {
        let random_char = get_random_letter(false);
        assert!(random_char.is_ascii_alphabetic());
        assert!(random_char.is_ascii_lowercase());

        for _ in 0..100 {
            let random_char = get_random_letter(true);
            if random_char.is_ascii_uppercase() {
                assert!(true);
                return;
            }
        }
    }
}
