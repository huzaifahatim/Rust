#[derive(Debug)]
struct AREA {
    length: u32,
    breath: u32,
    height: u32,
}    
/*impl AREA {
    fn area (&self) -> u32 {
        self.breath * self.height * self.length
        
    }
}*/
//uSING STRUCT METHOD      

pub fn run () {
    let area1 = AREA {
        length: 20,
        breath: 30,
        height: 50,
    };

    println!("The area is {} square pixels.",area(&area1));
    println!("The component of area is {:?} .", area1 );//Using  Debug trait

    fn area(area: &AREA) -> u32 {
        area.breath * area.height * area.length
    }    

        
}
