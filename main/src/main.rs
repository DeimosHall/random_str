use random_str as random;

fn main() {
    let random_letter = random::get_random_letter(true, true);
    println!("Random letter: {}", random_letter);
    
    let random_symbol = random::get_random_symbol();
    println!("Random symbol: {}", random_symbol);

    let random_int = random::get_random_int(0, 9);
    println!("Random int: {}", random_int);

    let random_phone_number = random::get_random_int(1, 10000000);
    println!("Random phone number: +52 343{}", random_phone_number);

    let random_string = random::get_random_string(16, true, false);
    println!("Random string: {}", random_string);
}
