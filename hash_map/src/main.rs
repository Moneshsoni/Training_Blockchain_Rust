use std::collections::HashMap;
fn main() {
    let mut marks = HashMap::new();
    //add value in hash using
    marks.insert("Rust programming Language", 98);
    marks.insert("C programming Language", 98);
    marks.insert("C++ programming Language", 98);
    marks.insert("Java programming Language", 98);
    //get a length of hashmap
    println!("Length of hashmap !{}",marks.len());
    //To get a single value of hashmap
    match marks.get("Java programming Language"){
        Some(mark) =>println!("You got !  Java {}",mark),
        None => println!("you didn't study java"),
    }

    //Remove value in hashmap
    marks.remove("C programming Language");
    //Loop through hashmap
    for (subject ,mark) in &marks{
        println!("For subjec {} you got number {}%",subject,mark);
    }


    //check value exiest or not 
    println!("Did you study C++? {} ",marks.contains_key("C++ programming Language"));

}
