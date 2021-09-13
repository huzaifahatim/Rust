# DATA TYPES

Like every other language, every value in **RUST** is of certain data type.

THERE ARE TWO TYPES OF DAA TYPES:

* SCALAR TYPES
* COMPOUND TYPES

## SCALAR TYPES

A scalar type represents a single value. Rust has four primary scalar types:

* integer
* float
* boolean
* charracter

**INTEGER** An integer is a number without a fractional component. In integer we have to give its type. FOR EXAMPLE: If we write u32 this means that the value which we are giving is unsigned and of 32 bit.

```rust
fn main(){
    let a:i32 = 4
}
```

OTHERS ARE:

* signed/unsigned:

     i8 ,  u8
     i32 , u32
     i64 , u64
     i128 , u128

**FLOATING POPINT** Just like other floating points numbers are those which have deciaml points.Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

**BOOLEAN** As in most other programming languages, a Boolean type in Rust has two possible values: true and false

```rust

fn main() {
    let t = true;

    let f: bool = false; 
}
```

**CHARACTER** We use only single character in *char* which must be written in between '' ('char').

```rust
fn main() {
    let a = 'H';
}
```

## COMPOUND TYPE

* TUPLE
* ARRAY

**TUPLE** Is a way of grouping different type of data under one thing.Tuples have a fixed length: once declared, they cannot grow or shrink in size. We seprate each value by a comma (,) and we write those value between paranthesis and you can  access it by using indexing with (.).

```rust
fn main(){
    let a  = ("JOHN","JAMES" , "JACK" );//FOR STRING
    println!("MY NAME IS {}", a.0);
    let tup: (i32, f64, u8) = (500, 6.4, 1); //FOR NUMERICAL VALUE
    
}
```

**ARRAY** Like *TUPLE* Array is also of fixed length in Rust. We uses straight bracket and seprate value by using comma but we don't add differnt type of data. You can call it by using indexing samelike ijn any othe rprograming language

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let x = a[2];
}
