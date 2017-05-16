extern crate docopt;
extern crate microstatus;
extern crate rustc_serialize;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::Drain;

const USAGE: &str = "
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
        println!("microstatus v{}", microstatus::version());
    } else {
        let exit_code = microstatus::run(&args.arg_working_directory, &logger);
        if exit_code != 0 {
            drop(logger);
            std::process::exit(exit_code)
        }
    }
}
