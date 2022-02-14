fn main() {
    //Integer array collection
    //let arr=[1,2,3,4,56,6];
    // string array collection
    // let cl=["red","blue","black"];
    // for i in cl.iter(){
    //     println!("Array value is in string {}",i);
    // }
    let arr = [12,3,4,3,2];
    for i in arr.iter(){
        if i%3==0{
            if i==&3{
                break;
            }
            println!("Number is divide by  {}",i);
        }
        else{
            println!("Number is not divided by  {}",i);
        }
    }
}

