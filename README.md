# random_str

Generate random strings, chars, booleans, and integers.

## What can you do?

If you want to create random texts, phone numbers, or passwords, you can use random_str to do so.

Visit the Github repository: https://github.com/DeimosHall/random_str

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
```

Use `cargo run` to test the result.

```
cargo run
```

Possible output:

```
random_str on  main [!] via 🦀 v1.67.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/main`
Random letter: b
Random symbol: $
Random number: 7
Random phone number: +52 3431550168
Random password: ovnaLx1A%NCric1H
Random bool: true
Random char: ^
```

## How to contribute

If you think there is a function that may be helpful, you can open a pull request. Please, follow the next steps.

1. Make a fork of the repository.

<p align="center">
  <img src="https://drive.google.com/uc?id=1GmK9AIRnsOPea1ZJoWSdv71L9ouo4Bgp" style="border-radius: 10px;">
</p>

2. Clone the repositoy.

```
git clone https://github.com/DeimosHall/random_str.git
```

3. Change to the contribute branch.

```
git checkout contribute
```

4. Update the contribute branch

```
git merge main
```

5. Commit your changes using [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). For example:

```
git commit -m "feat: add my_method"
```

6. Push your changes to the contribute branch, I will check them and make the merge to the main branch.

```
git push origin contribute
```
