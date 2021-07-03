use clap::{Arg, App};

fn main() {
    let matches = App::new("My test program")
        .version("0.1")
        .author("idoplay123@163.com")
        .about("Exercise command line argument parsing.")
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A file name")
            )
        .arg(Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(true)
                .help("A number")
            )
        .get_matches();
    let file = matches.value_of("file").unwrap_or("input.txt");
    println!("The file is: {}", file);

    let num_str = matches.value_of("number");
    match num_str {
        None => println!("A number is needed."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your number is {}.", n),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}
