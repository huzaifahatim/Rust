//#[derive(Debug)]
/*struct AREA {
    length: u32,
    breath: u32,
    height: u32,
}*/
//Tuple Struct    
struct AREA (u32, u32, u32);    
  
/*impl AREA {
    fn area (&self) -> u32 {
        self.breath * self.height * self.length
        
    }
}*/
//uSING STRUCT METHOD      

pub fn run () {
    /*let area1 = AREA {
        length: 20,
        breath: 30,
        height: 50,
    };*/

    let area1 = AREA (20,30,50); //tuple struct

    //println!("The area is {} square pixels.",area(&area1));
    //println!("The component of area is {:?} .", area1 );//Using  Debug trait
    println!("The component of area is {}length , breath {} ,height {}", area1.0 , area1.1 ,area1.2); 

/*fn area(area: &AREA) -> u32 {
        area.breath * area.height * area.length
    }*/    

        
}

