use std::env;
use std::process;
use minigrep::Config; // Config struct within lib.rs

fn main() {
    let args: Vec<String> = env::args().collect();

    // These are some ways to parse env arg input
    let (_query, _file_path) = parse_config_by_ref(&args);
    //println!("Searching for {}", query);
    //println!("In file {}",file_path);
    // Class constructor is the preferred method to handle errors as we can catch multiple errors and pass them to an unwrap
    let config = Config::build(&args)
        .unwrap_or_else(
            |err| {
                eprintln!{"Problem passing arguments: {err}"};
                eprintln!{"Arguments provided:\n {:?}", args};
                process::exit(1);
            }
        );

    // Because run returns a () on success, which is the default output of main()
    // We do not need to unwrap it on success. Instead we can just use the following to raise an error

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}


fn parse_config_by_ref(args: &[String]) -> (&str,&str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path) // This gets coerced to an (&str,&str) type on return, the underlying str object
}