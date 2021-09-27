enum Movement {
    Up,
    Down,
    Left,
    Right,

}
fn move_avatar (m : Movement) {
    match m {
        Movement::Up =>println!("MOVING UP"),
        Movement::Down =>println!("MOVING DOWN",),
        Movement::Left =>println!("MOVING LEFT",),
        Movement::Right =>println!("MOVING RIGHT",),

    }
    


pub fn run () {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;
    

    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar1);
   }

   //option enum

    /*let some_number = Some(5);
    let some_string = Some("a string");





    let absent_number: Option<i32> = None;*/ //
//fn main() {
/*fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1) 
            => println!("{}",some(5))

        }
        
    }


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
//}*/




}

