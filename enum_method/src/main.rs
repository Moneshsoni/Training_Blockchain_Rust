//We remove all warning using this code
#![allow(dead_code)]

enum Day{
    Monday,Tuesday,Wenesday,Thursday,Friday,Saturday,Sunday
}
impl Day{
    fn is_weakday(&self)->bool{
        match self{
            &Day::Saturday|&Day::Sunday=> return false,
            _=> return true
        }
    }
}
fn main() {
    let d= Day::Sunda;
    println!("Is day a weekday: {}",d.is_weakday());
}
