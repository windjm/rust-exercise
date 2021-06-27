use std::fs;

fn main() {
    // TODO: 支持选项
    let dir = std::env::args().nth(2).expect("No directory given");

    for entry in fs::read_dir(dir).expect("Read directory failed.") {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }
}
