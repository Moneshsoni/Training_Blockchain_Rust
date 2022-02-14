struct Color{
    red:u32,
    blue:u32,
    black:u32,
}
fn main() {
    let blue = Color{
        red:120,
        blue:102,
        black:100,
    };
    print_colour(&blue)
}

fn print_colour(c: &Color){
    println!("Red {} Blue {} Black{}",c.red,c.blue,c.black)
}
