//Function definitions in Rust start with fn 
//and have a set of parentheses after the functio
pub fn run(){
    my_fun (8,12);
    
    fn my_fun(x:i32 , y:i32)  {
        let minus = x - y ;
        //return minus;
        println!("{}", minus); 
    }
    another_function(5);

    fn another_function(x: i32) {
        println!("The value of x is: {}", x);
    }
    let add = |a : i32 , b : i32 | a + b;
    println!("{}", add(5,7));

}
    


