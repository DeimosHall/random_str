use random_str::random::{self, CharBuilder, RandomCharBuilder, RandomStringBuilder};

fn main() {
    // Random letter including from 'a' to 'z' and from 'A' to 'Z'
    let random_letter: Option<char> = RandomCharBuilder::new()
        .with_lowercase()
        .with_uppercase()
        .build();
    println!("Random letter: {}", random_letter.unwrap());

    let random_symbol: Option<char> = RandomCharBuilder::new()
        .with_symbols()
        .build();
    println!("Random symbol: {}", random_symbol.unwrap());

    let min = 0;
    let max = 9;
    let random_number = random::get_int(min, max); // Between 0 and 9
    println!("Random number: {}", random_number);

    let digits: Option<String> = RandomStringBuilder::new()
        .with_length(10)
        .with_numbers()
        .build();
    let random_phone_number = format!("+52 {}", digits.unwrap());
    println!("Random phone number: {}", random_phone_number);

    let random_password: Option<String> = RandomStringBuilder::new()
        .with_length(32)  // Optional, 16 as default
        .with_lowercase()
        .with_uppercase()
        .with_numbers()
        .with_symbols()
        .build();
    println!("Random password: {}", random_password.unwrap());

    let random_bool = random::bool();
    println!("Random bool: {}", random_bool);
}
