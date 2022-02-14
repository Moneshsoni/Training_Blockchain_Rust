fn main() {
    let mut my_string=String::from("Hello this is testing string");
    //length function
    println!("Length: {}",my_string.len());
    //Is empty
    println!("Is empty ? {}",my_string.is_empty());

    for token in my_string.split_whitespace(){
        println!("Split word {}",token);
    }

    println!("Does this string contain 'Hello'?{}",my_string.contains("Hello"));
    my_string.push_str(" Welcome to string of saturday ");
    println!("Updated string is {} ",my_string);
}
