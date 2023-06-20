//! This crate provides a set of functions to generate random strings, numbers, letters, symbols and booleans.
use rand::prelude::*;

fn get_symbols_list() -> Vec<char> {
    vec!['#', '$', '%', '&', '*', '@', '^', '!']
}

/// Get a random char from a-z or A-Z
/// 
/// # Panics
/// Panics if the function is called with both parameters set to false
///
/// # Examples
///
/// ```
/// use random_str as random;
/// ```
/// 
/// ```
/// let random_letter = random::get_letter(true, false);
/// println!("Random letter: {}", random_char);
/// ```
/// Possible output: Random letter: w
/// ```
/// let random_letter = random::get_letter(false, true);
/// println!("Random letter: {}", random_letter);
/// ```
/// Possible output: Random letter: X
/// ```
/// let random_letter = random::get_letter(true, true);
/// println!("Random letter: {}", random_letter);
/// ```
/// Possible output: Random letter: x or Random letter: Y
#[cfg(not(doctest))]
pub fn get_letter(lowercase: bool, uppercase: bool) -> char {
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

/// Get a random number (i32) from a minimum and maximum value
/// 
/// # Panics
/// Panics if min is greater than max
/// 
/// # Examples
/// 
/// ```
/// use random_str as random;
/// ```
/// ```
/// let random_number = random::get_int(0, 9);
/// println!("Random number: {}", random_number);
/// ```
/// Possible output: Random number: 3
#[cfg(not(doctest))]
pub fn get_int(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

/// Get a random symbol from a list of symbols
/// Possible symbols are: #, $, %, &, *, @, ^
/// 
/// # Examples
/// 
/// ```
/// use random_str as random;
/// ```
/// ```
/// let random_symbol = random::get_symbol();
/// println!("Random symbol: {}", random_symbol);
/// ```
#[cfg(not(doctest))]
pub fn get_symbol() -> char {
    let symbols = get_symbols_list();
    let mut rng = thread_rng();
    let random_symbol = symbols.choose(&mut rng).unwrap();
    *random_symbol
}

/// Get a random string with a given length and a set of characters
/// 
/// # Panics
/// Panics if the function is called with all parameters set to false
/// 
/// # Examples
/// 
/// ```
/// use random_str as random;
/// ```
/// ```
/// let random_password = random::get_string(16, true, true, true, true);
/// println!("Random password: {}", random_password);
/// ```
#[cfg(not(doctest))]
pub fn get_string(length: usize, lowercase: bool, uppercase: bool, numbers: bool, symbols: bool) -> String {
    let symbols_list: Vec<char> = get_symbols_list();
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
    if numbers {
        random_string.extend(numbers_list);
    }
    if symbols {
        random_string.extend(symbols_list);
    }

    let mut rng = thread_rng();
    (0..length).map(|_| *random_string.choose(&mut rng).unwrap()).collect()
}

/// Get a random boolean value
/// 
/// # Examples
/// 
/// ```
/// use random_str as random;
/// ```
/// ```
/// let random_bool = random::get_bool();
/// println!("Random boolean: {}", random_bool);
/// ```
/// Possible output: Random boolean: true or Random boolean: false
#[cfg(not(doctest))]
pub fn get_bool() -> bool {
    let mut rng = thread_rng();
    rng.gen_bool(0.5)
}

pub fn get_char() -> char {
    let chars: Vec<char> = (b'!'..b'~').map(|c| c as char).collect();
    let mut rng = thread_rng();
    chars.choose(&mut rng).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_lowercase_letter() {
        let random_letter = get_letter(true, false);
        assert!(random_letter.is_ascii_alphabetic());
        assert!(random_letter.is_ascii_lowercase());
    }

    #[test]
    fn test_get_random_uppercase_letter() {
        let random_letter = get_letter(false, true);
        assert!(random_letter.is_ascii_alphabetic());
        assert!(random_letter.is_ascii_uppercase());
    }

    #[test]
    fn test_get_random_letter() {
        let random_letter = get_letter(true, true);
        assert!(random_letter.is_ascii_alphabetic());
    }

    #[test]
    // Validate if the function panics when both parameters are false
    fn test_get_random_false_letter() {
        let result = std::panic::catch_unwind(|| get_letter(false, false));
        assert!(result.is_err());
    }
}
