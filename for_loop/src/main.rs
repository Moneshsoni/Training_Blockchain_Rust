fn main() {
    // for i in 1..11{
    //     println!("The value of i is {}",i);
    // }
    // let number=[1,2,4,54,45,4,45];
    // for i in number{
    //     println!("Array value is {}",i);
    // }
    //We use iter() with vector value because due to ownership 
    //vector with integer
    // let number=vec![1,2,4,54,45,4,45];
    // for i in number.iter(){
    //     println!("Array value is {}",i);
    // }
    
    // let animal = vec!["rabit","dog","cat","monkey"];
    // for a in animal.iter(){
    //     if a==&"cat"{
    //         break;
    //     }
    //     println!("The animal name is {}",a);
    // }


    //For loop with tuple 
    let animal = vec!["rabit","dog","cat","monkey"];
    for (index,a) in animal.iter().enumerate(){
        println!("The Index is {} The animal is {}",index,a);
    }
}
    // let number=vec![1,2,4,54,45,4,45];
    // for i in number.iter(){
    //     println!("Array value is {}",i);
    // }