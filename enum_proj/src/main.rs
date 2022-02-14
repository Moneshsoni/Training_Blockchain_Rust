enum Rust{
    Up,
    Down,
}
fn main() {
    let lan_dir:Rust = Rust::Down;
    match lan_dir{
        Rust::Up=> println!("Hello I am Up method "),
        Rust::Down=> println!("Hello I am Down enum method "),
    }
}