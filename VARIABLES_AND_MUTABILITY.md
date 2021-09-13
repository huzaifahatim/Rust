# VARIABLES

As by default variable are immutalemeans that they cannot change.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Save and run the program using cargo run. You should receive an error message, as shown in this output:

```rust
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

error: aborting due to previous error
```

`This shows that you cannot assign value two times to variable`

## **MUTABILITUY**

YOU CAN MAKE A VARIABLE MUTABLE BY ADDING A **mut** KEYWORD TO THE VARIABLE.

```rust
fn main() {
    let x mut = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}  
```

Now if we run program.

```rust
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```
