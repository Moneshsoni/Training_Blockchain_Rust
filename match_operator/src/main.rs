fn main() {
    //match is just like a switch statement in other language
    //The check the value and return the value
   
   //* For integer value*
    // let number =10;
    // match number {
    //     1 => println!("The number is 10"),
    //     2 => println!("The number is 2"),
    //     //check either number x or y like pattern
    //     // 10|11 => println!("The number is either 10 or 11"),

    //     //get value on range between
    //     1..=20 => println!("The value is under range {}",number),
    //     _ => println!("It does'nt match"),


    // }

    //match keyword with string
    let name ="monesh";
    match name{
        "sanjay"=>println!("This is common name"),
        "monesh"=>println!("Great name monesh bro"),
        _ => println!("name doesn't match on list"),
    }
}
