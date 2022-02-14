mod test{
    //private method in module
    fn dinner(){
        println!("Today is chicken dinner ! for my private method side ");
    }
    pub fn mod_test(){
        
        println!("Hello this is module test ");
       //calling private method
        dinner();
    }
    //define module under other module
    pub mod inner_mod{
        pub fn inn_module(){
            println!("I am the inner module method");
        } 
    }
}

fn main() {
    test::mod_test();
    println!("Hello, From man function");
    test::inner_mod::inn_module();
}
