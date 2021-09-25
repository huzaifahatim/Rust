pub fn run () {
    let a = 5;
    println!("{}", a);


    //mutable 
    let mut b  = 6;
    println!("{}",b);
    let b = 7;
    println!("{}",b);

    //constant
    const C : i32 = 5;//it can never be changed not by mut
    println!("{}",C);

    //shadowing
    let d = 55;
    let d = d * 2;
    println!("{}",d);

    

}

