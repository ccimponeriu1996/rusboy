use std::env;
use std::io::Read;
use std::fs;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    println!("{}", file_name);

    let mut file = fs::File::open(&file_name).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    let file_buf = file_buf;
    println!("{:?}", file_buf);
}
