

pub fn largest <T: PartialOrd + Copy>(list: &[T]) -> T {

    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Number_list <T,E> {
    x : T, 
    y : E,
}  

impl Number_list<T,E> {
    fn distance_from_origin(&self) -> <T,E> {
        (self.x.powi(2) + self.y.powi(2)).sqrt()

    }
    
}


pub fn main() {
    let value =  Number_list {x:25.2 ,y: 742};
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);


    
}



