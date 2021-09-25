//There are two types of loop
    //while conditional loop
    //for loop

     
pub fn run(){

    let mut a : i32= 33;
    
    while a >= 20 {
        a = a-2;
        println!("NUMBER {}", a);
    }

    let x  = 0 ;
    for x in 0..100 {

        if x % 15 == 0 {
            println!("FIZZBUZZ");    
        }
        else if x % 3 == 0{
            println!("FIZZ");
        }
        else if x % 5 == 0 {
            println!("BUZZ");
        }
        else {
            println!("NUMBER {}", x);
        }
    }
}


