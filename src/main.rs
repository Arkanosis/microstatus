extern crate docopt;
extern crate rustc_serialize;

const USAGE: &'static str = "
Usage: microstatus <working-directory>
       microstatus -h | --help
       microstatus --version

Arguments:
    working_directory working directory (file storage)

Options:
    -h, --help  Show this screen.
    --version   Show version.
";

#[derive(RustcDecodable)]
struct Args {
    arg_working_directory: String,
    flag_version: bool,
}

fn main() {
    let args: Args =
        docopt::Docopt::new(USAGE)
            .and_then(|docopts|
                docopts.argv(std::env::args().into_iter())
                   .decode()
            )
            .unwrap_or_else(|error|
                error.exit()
            );

    if args.flag_version {
        println!("microstatus v{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
    } else {
        println!("Working directory: {}", args.arg_working_directory);
    }
}
