fn main() {
    let mut n=0;
    loop{
        n=n+1;
        if n==5 {
            continue;
        }
        if n>10{
            break;
        }
        println!("Multiply of  is {}",n*2);
    }
}
