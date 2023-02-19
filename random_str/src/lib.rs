use rand::prelude::*;

/// Get a random char from a-z or A-Z
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
#[cfg(not(doctest))]
pub fn get_random_letter(lowercase: bool, uppercase: bool) -> char {
    let mut chars: Vec<char> = vec![];

    if lowercase {
        let lower_chars: Vec<char> = (b'a'..b'z').map(|c| c as char).collect();
        chars = lower_chars;
    }
    
    if uppercase {
        let capital_chars: Vec<char> = (b'A'..b'Z').map(|c| c as char).collect();
        chars.extend(capital_chars);
    }

    let mut rng = thread_rng();
    let random_char = chars.choose(&mut rng).unwrap();
    *random_char
}

pub fn get_random_int(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

pub fn get_random_symbol() -> char {
    let symbols: Vec<char> = vec!['#', '$', '%', '&', '*', '@', '^'];
    let mut rng = thread_rng();
    let random_symbol = symbols.choose(&mut rng).unwrap();
    *random_symbol
}

pub fn get_random_string(length: usize, lowercase: bool, uppercase: bool, number: bool, symbol: bool) -> String {
    let symbols_list: Vec<char> = vec!['#', '$', '%', '&', '*', '@', '^'];
    let numbers_list: Vec<char> = (b'0'..b'9').map(|c| c as char).collect();
    let cap_letters_list: Vec<char> = (b'A'..b'Z').map(|c| c as char).collect();
    let low_letters_list: Vec<char> = (b'a'..b'z').map(|c| c as char).collect();
    let mut random_string: Vec<char> = vec![];

    if lowercase {
        random_string.extend(low_letters_list);
    }
    if uppercase {
        random_string.extend(cap_letters_list);
    }
    if number {
        random_string.extend(numbers_list);
    }
    if symbol {
        random_string.extend(symbols_list);
    }

    let mut rng = thread_rng();
    (0..length).map(|_| *random_string.choose(&mut rng).unwrap()).collect()
}

pub fn get_random_bool() -> bool {
    let mut rng = thread_rng();
    rng.gen_bool(0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_lowercase_letter() {
        let random_letter = get_random_letter(true, false);
        assert!(random_letter.is_ascii_alphabetic());
        assert!(random_letter.is_ascii_lowercase());
    }

    #[test]
    fn test_get_random_uppercase_letter() {
        let random_letter = get_random_letter(false, true);
        assert!(random_letter.is_ascii_alphabetic());
        assert!(random_letter.is_ascii_uppercase());
    }

    #[test]
    fn test_get_random_letter() {
        let random_letter = get_random_letter(true, true);
        assert!(random_letter.is_ascii_alphabetic());
    }

    #[test]
    // Validate if the function panics when both parameters are false
    fn test_get_random_false_letter() {
        let result = std::panic::catch_unwind(|| get_random_letter(false, false));
        assert!(result.is_err());
    }
}
