use random_str as random;

fn main() {
    let lowercase = true;
    let uppercase = true;

    let random_letter = random::get_letter(lowercase, uppercase);
    println!("Random letter: {}", random_letter);

    let random_symbol = random::get_symbol();
    println!("Random symbol: {}", random_symbol);

    let mut min = 0;
    let mut max = 9;
    let random_number = random::get_int(min, max);
    println!("Random number: {}", random_number);

    min = 1000000;
    max = 9999999;
    let seven_digits = random::get_int(min, max); // 7 digits
    let random_phone_number = format!("+52 343{}", seven_digits);
    println!("Random phone number: {}", random_phone_number);

    let length = 16;
    let numbers = true;
    let symbols = true;
    let random_password = random::get_string(length, lowercase, uppercase, numbers, symbols);
    println!("Random password: {}", random_password);

    let random_bool = random::get_bool();
    println!("Random bool: {}", random_bool);

    let random_char = random::get_char();
    println!("Random char: {}", random_char);
}
