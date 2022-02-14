fn main() {
    //outside scope
    let x = 100;
    {
        //Inner scope 
        let y =200;
        println!("X is {} Y is {}",x,y);
    }
    // We can't access Inner scope value in outside scope
    //println!("X is {} Y is {}",x,y);

}
