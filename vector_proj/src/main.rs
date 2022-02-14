fn main() {
    let mut my_vector =vec![12,3,43,45,6];
    //If we want to use push and remove method we need
    // make vector mutable
    //get length of vector
    let len=my_vector.len();
    println!("length of vector {}",len);
    //push new element in vector
    my_vector.push(65);
    my_vector.remove(1);

    for number in my_vector.iter(){
        println!("My vector number is {}",number);
    }
}
