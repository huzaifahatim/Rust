use std::{env::var, ffi::VaList};




pub fn largest <T: PartialOrd + Copy>(list: &[T]) -> T {

    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
#[derive(Debug)]
struct NumberList <T> {
    x: T, 
    y : T,
    z : i32,
    
}

impl <T> NumberList<T> {
    fn add<T: std::ops::Add<Output = T>>(&self : value) -> NumberList<T> { 
        self.x.add() + self.y.add()
    } 
    

}




pub fn main() {
    let value =  NumberList {x: 25.2 ,y: 74.5, z: 44};
    println!("the value of x is {} y is {} and z is {}" , value.x , value.y ,value.z );
    println!("ADDITION {}",add(&self));

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);


    
}    
    






