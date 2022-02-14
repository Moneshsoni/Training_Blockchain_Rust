//Function with odd and even
fn main() {
   number_is(10);
}

fn number_is(num:u32){
    for i in 1..num{
        if is_even(i){
            println!("Number is even {} ",i);
        }
        else{
            println!("Number is odd {} ",i);
        }

    }
} 

fn is_even(num:u32) -> bool {
    return num%2==0
}