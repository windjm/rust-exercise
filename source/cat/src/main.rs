fn main() {
    let filename = std::env::args().nth(2).expect("No file given");
    let contents = std::fs::read_to_string(filename).expect("Read file failed.");
    println!("{}", contents);
}
