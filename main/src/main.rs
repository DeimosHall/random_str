use random_str as random;

fn main() {
    let random_letter = random::get_letter(true, true);
    println!("Random letter: {}", random_letter);

    let random_symbol = random::get_symbol();
    println!("Random symbol: {}", random_symbol);

    let random_number = random::get_int(0, 9);
    println!("Random number: {}", random_number);

    let seven_digits = random::get_int(1000000, 9999999); // 7 digits
    let random_phone_number = format!("+52 343{}", seven_digits);
    println!("Random phone number: {}", random_phone_number);

    let random_password = random::get_string(16, true, true, true, true);
    println!("Random password: {}", random_password);

    let random_bool = random::get_bool();
    println!("Random bool: {}", random_bool);

    let random_char = random::get_char();
    println!("Random char: {}", random_char);
}
