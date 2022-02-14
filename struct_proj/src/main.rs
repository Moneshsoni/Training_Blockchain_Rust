struct School{
    name:u32,
    surname:u32,
    number:u32,

}


fn main() {
    let x= School{
        name:12,
        surname:32,
        number:12343,
    };
    println!("Name is {}",x.name);
    println!("surname is {}",x.surname);
    println!("number is {}",x.number);
} 
