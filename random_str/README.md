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
random_str = "0.1.0"
```

Use `cargo build` to download the dependency.

```
cargo build
```

In your main.rs file write the following:

```
use random_str;

fn main() {
    let random_char = random_str::get_random_char(true);
    println!("Random char: {}", random_char);
}
```

Use `cargo run` to test the result.

```
cargo run
```

> Note: the get_random_char method the is only one available right now, tomorrow I will add more :D
