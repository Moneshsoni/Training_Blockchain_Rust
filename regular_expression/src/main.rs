extern crate regex;
use regex::Regex;
fn main() {
    let re= Regex::new(r"(\w{5})").unwrap();
    let text= "mon";
    match re.captures(text){
        Some(m)=>println!("Regular expression match {}",&m[1]),
        None => println!("Regular expression not match "),
    }
}
