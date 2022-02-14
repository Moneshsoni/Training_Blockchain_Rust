struct Num(u32,u32,u32);
fn main() {
    let mut tup1 = Num(230,43,4);
    tup1.2=400;
    println!("first {} second {} third {} tuple ",tup1.0,tup1.1,tup1.2);
}
