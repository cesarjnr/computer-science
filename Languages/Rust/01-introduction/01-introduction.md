# Introduction

## Installation

In order to install Rust, we're going to use `rustup`, which manages installed Rust versions and associated tools. To install `rustup`, run the following command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If the installation is successful, the following message will be displayed on the terminal:

```
Rust is intalled now. Great!
```

Restart your terminal and then check if it's working:

```
rustc --version
```

## The `main()` Function

Create a new file called `hello_world.rs` and give it a `main()` function:

```rs
fn main() {
  println!("Hello World!");
}
```

In Rust, you have to put all your executable code inside functions. The `main()` function in Rust is special. It is the default starting point when you run your program. When you return from the `main()` function, the program exits and passes control back to the operating system.
Rust is a compiled language. That means we need to run our programa through a Rust compiler to turn it into an executable file. We then run that executable like we would run any other program on the command line.
To compile `hello_world.rs`, run the following in your terminal:

```
rustc hello_world.rs
```

If no errors occurs, this will produce an executable named `hello_world`. To run `hello_world`, type `./hello_world` and press `Enter`.