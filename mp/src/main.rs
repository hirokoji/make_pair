mod lib;
use lib::{cli, files};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about="how to use struct-opt crate")]
pub struct Opts{
    #[structopt(subcommand) ]
    subcommands: Sub
}

#[derive(Debug, StructOpt)]
#[structopt(name = "sub", about = "sub commands")]
enum Sub {
    #[structopt(name = "assign")]
    Assign(AssignOpts),
    #[structopt(name = "history")]
    History
}

#[derive(Debug, StructOpt)]
struct AssignOpts {
    members: String,
}


fn main() {
    let opts = Opts::from_args();

    let home = std::env::var("HOME").unwrap();
    let history_dir = home + "/.mp";
    let history = files::History::new(&history_dir, "mp_history");

    match opts.subcommands {
        Sub::Assign(opts)  => cli::assign_cmd(opts.members, history),
        Sub::History =>  cli::history_cmd(history)
    }
}
