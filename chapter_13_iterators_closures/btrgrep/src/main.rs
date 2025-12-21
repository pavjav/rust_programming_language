use std::env;
use std::process;
use btrgrep::Config; // Config struct within lib.rs

fn main() {
    // We modify this build method to take an env::args() iterator

    let config = Config::build(env::args())
        .unwrap_or_else(
            |err| {
                eprintln!{"Problem passing arguments: {err}"};
                process::exit(1);
            }
        );

    // Because run returns a () on success, which is the default output of main()
    // We do not need to unwrap it on success. Instead we can just use the following to raise an error

    if let Err(e) = btrgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

#[allow(dead_code)]
fn parse_config_by_ref(args: &[String]) -> (&str,&str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path) // This gets coerced to an (&str,&str) type on return, the underlying str object
}