//There are two types of loop
    //while conditional loop
    //for loop

     
pub fn run(){

    let mut a : i32= 33;
        
    
    while a >= 20 {
        a = a-2;
        println!("NUMBER {}", a);
    }

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


    //Loop
    let mut count = 0;
    
    loop {
        count += 1;
        println!("NUMBERS:{} ",count);

        if count == 20 {
            break;
        }

            
    
    }
}




