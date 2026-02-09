use random_str::{self as random, CharBuilder, RandomCharBuilder, RandomStringBuilder};

fn main() {
    // Random letter including from 'a' to 'z' and from 'A' to 'Z'
    let random_letter = RandomCharBuilder::new()
        .with_lowercase()
        .with_uppercase()
        .build();
    println!("Random letter: {}", random_letter.unwrap());

    let random_symbol = RandomCharBuilder::new()
        .with_symbols()
        .build();
    println!("Random symbol: {}", random_symbol.unwrap());

    let mut min = 0;
    let mut max = 9;
    let random_number = random::get_int(min, max);
    println!("Random number: {}", random_number);

    min = 1000000;
    max = 9999999;
    let seven_digits = random::get_int(min, max); // 7 digits
    let random_phone_number = format!("+52 343{}", seven_digits);
    println!("Random phone number: {}", random_phone_number);
    
    let random_password = RandomStringBuilder::new()
        .with_length(32)  // Optional, 16 as default
        .with_lowercase()
        .with_uppercase()
        .with_numbers()
        .with_symbols()
        .build();
    println!("Random password: {}", random_password.unwrap());

    let random_bool = random::get_bool();
    println!("Random bool: {}", random_bool);

    let random_char = random::get_char();
    println!("Random char: {}", random_char);
}
