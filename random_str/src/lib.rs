//! This crate provides a set of functions to generate random strings, numbers, letters, symbols and booleans.
use std::sync::LazyLock;

use rand::seq::IndexedMutRandom;

static LOWERCASE: LazyLock<Vec<char>> = LazyLock::new(|| {
    (b'a'..b'z').map(|c| c as char).collect()
});
static UPPERCASE: LazyLock<Vec<char>> = LazyLock::new(|| {
    (b'A'..b'Z').map(|c| c as char).collect()
});
static NUMBERS: LazyLock<Vec<char>> = LazyLock::new(|| {
    (b'0'..b'9').map(|c| c as char).collect()
});
static SYMBOLS: LazyLock<Vec<char>> = LazyLock::new(|| {
    vec!['#', '$', '%', '&', '*', '@', '^', '!']
});

// TODO: delete
fn get_symbols_list() -> Vec<char> {
    vec!['#', '$', '%', '&', '*', '@', '^', '!']
}

pub trait CharBuilder {
    fn options(&mut self) -> &mut Vec<char>;

    fn with_lowercase(mut self) -> Self where Self: Sized {
        self.options().extend(LOWERCASE.iter());
        self
    }

    fn with_uppercase(mut self) -> Self where Self: Sized {
        self.options().extend(UPPERCASE.iter());
        self
    }

    fn with_numbers(mut self) -> Self where Self: Sized {
        self.options().extend(NUMBERS.iter());
        self
    }

    fn with_symbols(mut self) -> Self where Self: Sized {
        self.options().extend(SYMBOLS.iter());
        self
    }
}

pub struct RandomCharBuilder {
    options: Vec<char>
}

impl CharBuilder for RandomCharBuilder {
    fn options(&mut self) -> &mut Vec<char> {
        &mut self.options
    }
}

impl RandomCharBuilder {
    pub fn new() -> Self {
        RandomCharBuilder { options: Vec::new() }
    }

    pub fn build(mut self) -> Option<char> {
        if self.options.is_empty() {
            return None;
        }

        let mut rng = rand::rng();
        self.options.choose_mut(&mut rng).copied()
    }
}

pub struct RandomStringBuilder {
    options: Vec<char>,
    length: usize
}

impl CharBuilder for RandomStringBuilder {
    fn options(&mut self) -> &mut Vec<char> {
        &mut self.options
    }
}

impl RandomStringBuilder {
    pub fn new() -> Self {
        RandomStringBuilder { options: Vec::new(), length: 16 }
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn build(mut self) -> Option<String> {
        if self.options.is_empty() {
            return None;
        }

        let mut rng = rand::rng();
        Some((0..self.length).map(|_| *self.options().choose_mut(&mut rng).unwrap()).collect())
    }
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
#[deprecated(since = "1.0.0", note = "Use RandomCharBuilder instead")]
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

    let mut rng = rand::rng();
    let random_char = chars.choose_mut(&mut rng).unwrap();
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
    use rand::RngExt;

    let mut rng = rand::rng();
    rng.random_range(min..=max)
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
    let mut symbols = get_symbols_list();
    let mut rng = rand::rng();
    let random_symbol = symbols.choose_mut(&mut rng).unwrap();
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
#[deprecated(since = "1.0.0", note = "Use RandomStringBuilder instead")]
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

    let mut rng = rand::rng();
    (0..length).map(|_| *random_string.choose_mut(&mut rng).unwrap()).collect()
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
    use rand::RngExt;

    let mut rng = rand::rng();
    rng.random_bool(0.5)
}

#[deprecated(since = "1.0.0", note = "It will be removed")]
pub fn get_char() -> char {
    let mut chars: Vec<char> = (b'!'..b'~').map(|c| c as char).collect();
    let mut rng = rand::rng();
    chars.choose_mut(&mut rng).unwrap().clone()
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
