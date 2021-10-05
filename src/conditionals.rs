// if/else onditional
pub fn run () {
    let age : i32 = 21;
    let check_id :bool = true;

    if age >= 18 && check_id {
        println!("YOU CAM DRIVE");
    }
    else if age < 18 && check_id {
        println!("SHOW YOUR ID");
    }
    else {
        println!("YOU ARE ARRESTED");
    }
}


