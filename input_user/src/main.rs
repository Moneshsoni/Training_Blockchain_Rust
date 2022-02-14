use std::io;
fn main() {
    let mut input = String::new();
    println!("Hey !Buddy say something ");
    match io::stdin().read_line(&mut input){
        Ok(_)=>{
            //when we want string in uppercase so we use method
            println!("Success you said! {}",input.to_uppercase());

        }
        Err(e)=>{
            println!("Oops something went wrong! {}",input);

        }
    }
}
