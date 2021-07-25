use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

    /// Set speed
    #[structopt(short, long, default_value = "42")]
    speed: String,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}