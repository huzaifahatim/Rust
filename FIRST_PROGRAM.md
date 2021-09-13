# STARTING OFF IN A RUST

You need to make a folder/directory in which all your projects will be kept as sub-folders. To make a directory named Projects:

FOR WINDOWS IN CMD:

> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world

## WRITING AND RUNNING A FIRST PROGRAM

Next, make a new source file and call it main.rs (in src) Rust files always end with the .rs extension. If you’re using more than one word in your filename, use an underscore to separate them.For example First_program

Now open your main.rs and write your first program and save it.

```rust
fn main() {
    println!("HELLO WORLD");
}
```

*Congratulation* You have written yopurfirst program.now yopu have to run it,

write **cargo run** in your vsc code terminal

```rust
HELLO WORLD
```
