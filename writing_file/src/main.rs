use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::create("output.txt").expect("Could not create file");
    file.write_all(b"Welcome to RustMania For side Monesh soni Blockchain Developer")
        .expect("can not able to write mate sorray for that ");
}
