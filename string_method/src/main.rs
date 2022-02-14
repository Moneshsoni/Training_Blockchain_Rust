fn main() {
    //String replace method
    {   
        let my_string = String::from("Rust is !Fantastic Language");
        println!("String replace method:  {}",my_string.replace("Fantastic", "Great"));

    }
    /* Linesh 
    method in String*/
    {
        let my_string= String::from("Rust is \n Nice \n Language");
        // println!("{}",my_string);
        for line in my_string.lines(){
            println!("String line by line: [{}]",line);
        }
    }
    /* Split method in Rust */
    {
        let my_string= String::from("Rust+is+greate+Language");
        let token: Vec<&str> = my_string.split("+").collect();
        println!("{}",my_string);
        println!("At Index 2: {}",token[2]);
    }
    /*Trim method in String */

    {
        let my_string=String::from("       This is Rust Language \n ");
        println!("Before Trim: {}",my_string);
        println!("After Trim: {}",my_string.trim());
    }
    /* Chars method in string */
    {
        let my_string = String::from("The Rust Language");
        println!("{}",my_string);
        match my_string.chars().nth(500){
            Some(m)=>println!("String index value is : {}",m),
            None =>println!("This index chars not found:"),
        }
    }
}
