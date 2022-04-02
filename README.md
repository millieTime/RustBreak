# RustBreak

My roommate has a game that he made, and he wants it to be digitized. I figured it'd be a fun challenge, and I could make it even more productive by teaching myself a programming language. So this project is for my to learn Rust.

The game is semi-similar to chess, but on a 12x12 board. Almost every piece has a special role, and can be upgraded by cards, and there are some pieces that can enable you to move multiple pieces on a single turn. It's really interesting.

[Software Demo Video](https://youtu.be/h-yL3e-0jHA)

# Development Environment

I used Visual Studio Code with Rust and Cargo. Rust is the language of choice, and Cargo is a build tool for Rust that helps streamline project creation and development.

Rust is an interesting language with a strict compiler. It helps catch a lot of bugs before they can even get root, and helps prevent bugs from being able to form. An important feature of Rust is variable ownership and editability. When declared, variables have to be set as either values (NO CHANGING) or variables (changeable). When passed to functions, those functions take ownership of any variable or value passed to them and don't give it back unless specifically told to.

One library this project depends on is array2d. It uses version 0.2.1.

# Useful Cargo Commands

cargo new <project name>     Sets up for a new project
cargo check                  Compiles your project
cargo build                  Compiles AND Creates an executable of your project
cargo run                    Compiles, Creates an executable of your project AND runs it

cargo build --release        Compiles AND creates an executable of your project WITH optimizations. Will take longer to build.

# Useful Websites

* [Tutorial](https://doc.rust-lang.org/book/jh02-00-guessing-game-tutorial.html)
* [Rust Documentation](https://doc.rust-lang.org/std/)
* [Stack Overflow](https://stackoverflow.com)

# Future Work

* Entering coordinates backwards appears to pass anyway. Could be a feature, could be a bug, but I have to decide.
* Figure out how to allow pieces to move orthogonally or diagonally at the cost of one movement.
* Next step is to handle pieces with different trait values.
* Then implement command pieces that can control other pieces as well.