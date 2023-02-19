use random_str;

fn main() {
    let random_char = random_str::get_random_char(true);
    println!("Random char: {}", random_char);

    let rnd_str = random_str::get_random_string(16, true);
    println!("Random string: {}", rnd_str);
}
