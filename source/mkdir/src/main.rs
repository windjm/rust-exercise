use std::fs;

fn main() -> std::io::Result<()> {
    let dir_name = std::env::args().nth(2).expect("A directory name is needed.");
    fs::create_dir(dir_name)?;
    Ok(())
}