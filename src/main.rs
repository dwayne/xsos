use structopt::StructOpt;
use xsos::cli::{ self, Config };

fn main() {
    cli::run(Config::from_args())
}
