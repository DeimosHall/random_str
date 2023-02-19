use random_str as random;

fn main() {
    let random_char = random::get_random_char(true);
    println!("Random char: {}", random_char);

    let random_int = random::get_random_int(1, 100);
    println!("Random int: {}", random_int);

    let random_phone_number = random::get_random_int(1, 10000000);
    println!("Random phone number: +52 343{}", random_phone_number);

    let random_symbol = random::get_random_symbol();
    println!("Random symbol: {}", random_symbol);

    let rnd_str = random::get_random_string(16, true);
    println!("Random string: {}", rnd_str);
}
