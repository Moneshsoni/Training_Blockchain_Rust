struct Person{
    name:String,
    age:u8
}

trait HashVoiceBox{
    //
    fn speak(&self);
    //check if can speak
    fn can_speak(&self)-> bool;
}

impl HashVoiceBox for Person{
    fn speak(&self){
        println!("This person are able to speak {} ",self.name);
    }

    fn can_speak(&self) -> bool{
        if self.age>0{
            return true;
        }return false;

    }
}
fn main() {
    let person = Person{
        name:String::from("monesh"),
        age:12,
    };
    println!("Can {} this person can speak ! {}",person.name,person.can_speak());
    person.speak();
}
