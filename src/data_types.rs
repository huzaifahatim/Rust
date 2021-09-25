pub fn run () {
     //Integer
     let a : i8 = 4;//i = signed bit includes 8,16,32,64 (-,+)
     println!("{}",a);
     let a : u8 = 4;//u = unsigned bit includes 8,16,32,64
     println!("{}",a);
     
     //Float
     let a : f32 = 4.00; //f = float bit includes 32,64
     println!("{}",a);
     //char
     let a = 'A'; //it only indulge one character with single inverted commas
     println!("{}",a);
     //Boolean
     let _a : bool = true;
     println!("Boolean");

     //Tuple
      let tup: (&str, i64) = ("Huzaifa", 45);//Tuple can have any data type used ()this
      println!("{:?}", tup);
     //Arrays 
     let arr :[i64;4] = [54,53,52,51];//Arrays are fixed n size,used 
    
}
