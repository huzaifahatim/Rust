//if/else onditional
public fn run() {

    fn main(){
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
    
        //while conditional
        let mut a : i32= 33;
    
        while a >= 20 {
            a = a-2;
            println!("NUMBER {}", a);
        }
        
    
    }

    
    
           
}

