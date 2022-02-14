fn main() {
    let mut x=10;
    {
       let x = 15;
    }
    //shadowing the word and getting different value in every time
    println!("X is {}",x);
    let x = "Monesh soni test ";
    println!("X is string {}",x);
    let x=true;
    println!("Let X is bool {}",x);

}
