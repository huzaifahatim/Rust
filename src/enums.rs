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

}
