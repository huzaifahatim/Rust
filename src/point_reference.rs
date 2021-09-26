//Point Refernce
//for primitive 
pub fn run () {
    let arr1 = [1,2,3,4];
    let arr2 = arr1 ;
    println!("ARRAY {:?}",(arr1,arr2));

//for Non-primitive
    let vec1 = vec! (1,2,3,4);
    let vec2 = &vec1;
    println!("VECTOR {:?}",(&vec1,vec2));



}

