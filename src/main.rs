extern crate docopt;
extern crate rustc_serialize;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::Drain;

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
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

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
        let directory: &std::path::Path = std::path::Path::new(&args.arg_working_directory);
        if directory.exists() {
            if !directory.is_dir() {
                error!(logger, "Working directory path \"{}\" does not actually target a directory", args.arg_working_directory);
                drop(logger);
                std::process::exit(1);
            }
        } else {
            info!(logger, "Creating working directory at \"{}\"", args.arg_working_directory);
            if !std::fs::create_dir(&args.arg_working_directory).is_ok() {
                error!(logger, "Unable to create directory \"{}\"", args.arg_working_directory);
                drop(logger);
                std::process::exit(1);
            }
        }
    }
}
