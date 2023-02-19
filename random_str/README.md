# random_str

Generate random strings, chars, booleans, and integers.

If you want to create random texts, phone numbers, or passwords, you can use random_str to do so.

## Requirements

You need to have installed cargo on your system.

If you are using Windows, download the installer from the official page
in the following link:

~~~
https://www.rust-lang.org/tools/install
~~~

If you are using macOS,
GNU/Linux, WSL or a UNIX based system just type:

~~~
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
~~~

## How to use it

Create a project.

```
cargo new my_project
```

In your `Cargo.toml` file, add the following dependency:

```
[dependencies]
random_str = "0.1.1"
```

Use `cargo build` to download the dependency.

```
cargo build
```

In your main.rs file write the following:

```
use random_str as random;

fn main() {
    let random_letter = random::get_random_letter(true, true);
    println!("Random letter: {}", random_letter);

    let random_symbol = random::get_random_symbol();
    println!("Random symbol: {}", random_symbol);

    let random_number = random::get_random_int(0, 9);
    println!("Random number: {}", random_number);

    let seven_digits = random::get_random_int(1000000, 9999999); // 7 digits
    let random_phone_number = format!("+52 343{}", seven_digits);
    println!("Random phone number: {}", random_phone_number);

    let random_password = random::get_random_string(16, true, true, true, true);
    println!("Random password: {}", random_password);
}
```

Use `cargo run` to test the result.

```
cargo run
```

Possible output:

```
random_str on  main [!] via 🦀 v1.67.0
❯ cargo run
   Compiling main v0.1.0 (/home/deimos/Software dev/rust/projects/random_str/main)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/main`
Random letter: b
Random symbol: $
Random number: 7
Random phone number: +52 3431550168
Random password: ovnaLx1A%NCric1H
```
