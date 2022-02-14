fn main() {
    let mut n=1;
    while n<=50{
        if n%5==0{
            if n==25{                
                break;                
            }
            println!("n is {}",n);
                       
        }       
        n=n+1;
    }
}
