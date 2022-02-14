struct Person{
    name:String,
    age:u32,
}

impl ToString for Person{
    fn to_string(&self) -> String{
        return format!("My name is {} my age is {}",self.name,self.age);
    }
}
fn main() {
    let per=Person{
        name:String::from("Monesh"),
        age:24,
    };
    println!("{}",per.to_string())
    // println!("my name is {}",per.name);
    // println!("my age is {}",per.age);
}
