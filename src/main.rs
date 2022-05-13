fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let file_name = args.get(0);

    let file_name = match file_name {
        Some(f) => f,
        None => {
            eprintln!("The filename wasn't specified.");
            std::process::exit(1);
        }
    };

    let contents = std::fs::read_to_string(&file_name);

    let contents = match contents {
        Ok(c) => c,
        Err(why) => {
            eprintln!("Error while reading file: {}", why);
            std::process::exit(1);
        }
    };

    println!("{}", contents);
}
