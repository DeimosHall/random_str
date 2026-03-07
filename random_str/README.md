# random_str

Generate random strings, chars, booleans, and numbers.

## What can you do?

If you want to create random texts, phone numbers, or passwords, you can use random_str to do so.

Visit the [GitHub repository](https://github.com/DeimosHall/random_str) or the [Codeberg mirror](https://codeberg.org/deimoshall/random_str).

## Requirements

You need to have installed cargo on your system.

If you are using Windows, download the installer from the official page
in the following link:

```bash
https://www.rust-lang.org/tools/install
```

If you are using macOS,
GNU/Linux, WSL or a UNIX based system just type:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## How to use it

Create a project.

```bash
cargo new my_project
```

In your `Cargo.toml` file, add the following dependency:

```
[dependencies]
random_str = "1.1.0"
```

Use `cargo build` to download the dependency.

```bash
cargo build
```

In your main.rs file write the following:

```rust
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
    
    let random_number = random::number(0..9);
    println!("Random number: {}", random_number);
    
    let another_random_number = random::number(1.0..2.0);
    println!("Random float number: {}", another_random_number);

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
```

Use `cargo run` to test the result.

```bash
cargo run
```

Possible output:

```bash
Random letter: C
Random symbol: #
Random number: 5
Random float number: 1.487004718282926
Random phone number: +52 11932115293
Random password: M29Fd*tamf3spJxxZOVZRUil!vBjycUHD
Random bool: true
```

## How to contribute

If you think there is a function that may be helpful, you can open a pull request. Please, write your commits using [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more information.

## Credits

Made with ♥️ by [DeimosHall](https://deimoshall.dev/).