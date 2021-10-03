

pub mod movies {
    pub fn play(a: String , b: String ,c: String ,d: String ) {
        println!("You Choose {},{},{},{}",a,b,c,d);
        
    }    
}

//Modules are private by default

mod netflix_series {
    fn series (a: String, b: String ,c: String ,d: String ) {
        let a := ["Alive" , "AnneFrank (Parallel Stories)" , "Ace the Hole" , "An Actor's Revenge"];
        println!("The Movie Starts With A ARE : {:?}", a);
        // let b : (String,String) = ("BARKING DOGS", "BADBOYS");
        // println!("The Movie Starts With B ARE : {:?}", b);
        let c = "CAT WITH TH DOGS";
        println!("The Movie Starts With C Is : {:?}", c);
        let d = "Doctor Srange";
        println!("The Movie Starts With D ARE : {:?}", d);

        

        
        
    }
}
/*use movies::play;
fn main() {
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);
}*/
//public module


  
 

        
        
        