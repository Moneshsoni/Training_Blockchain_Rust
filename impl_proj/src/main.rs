struct Rectangle{
    height:u32,
    width:u32,
}

impl Rectangle{
    fn print_rectangle(&self){
        println!("Rectangle: {} x {}",self.height,self.width);
    }
    fn is_square(&self) -> bool{
        self.height != self.width
    }

}
fn main() {
    let r=Rectangle{
        height:10,
        width:12,
    };
    // r.print_rectangle();
    println!("my rectangel is square {}",r.is_square());
}

