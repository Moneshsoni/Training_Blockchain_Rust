fn main() {
    // let tup1=(1,2,3,4,5);
    // println!("Tuple is {}",tup1.2);
    //Using this tuple we get inner value
    // let tup1=(1,2,3,4,5,(34,56,755));
    // println!("Get inner tuple value {}",(tup1.5).2);

    //Tuple with different data type
    let tup1=(12,33,"monesh");
    let (a,b,c)=tup1;
    println!("a is {}",a);
    println!("b is {}",b);
    println!("c is {}",c);  
}
