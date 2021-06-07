mod files;
mod helper;

use std::io;
use structopt::StructOpt;
use std::char;
use chrono::Utc;
use files::History;


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
        Sub::Assign(opts)  => assign_cmd(opts.members, history),
        Sub::History =>  history_cmd(history)
    }
}

fn assign_cmd(members: String, history: History){
    if !helper::validate_input_members(&members) {
        println!("Error: Unexpected input format. Please check your input members format.");
        println!("Usage: $ mp Hiro,Walter,Ian,Gabe");
        return;
    }

    let teams = helper::random_assign_teams(members);

    let mut alphabet:u32 = 65; // 'A'
    let mut results:String = String::from(format!("[{}] ", Utc::now().to_string()));

    for team in teams {
        let mut result = format!("Team {}: ", char::from_u32(alphabet).unwrap());
        for member in team { result.push_str(&(format!("{} ", member))) }

        print!("{}\n", result);
        results.push_str(&result);
        alphabet+=1;
    }
    ask_save_history(&mut results, history);
}

fn history_cmd(history:History){
    display_history(history)
}

fn display_history(history: History) {
    let mut lines = history.get_all_lines();
    lines.reverse();

    let mut max_num = 10;
    if lines.len() < max_num {
        max_num = lines.len();
    }

    for i in 0..max_num {
        println!("{}", lines[i]);
    }
}

fn ask_save_history(results: &mut String, history:History) {

    loop {
        println!("\nDo you save the result to history? [y/n]: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "y" | "yes" => {
                history.save(&results);
                println!("Saved the result");
                break;
            }
            "n" | "no" => { break; }
            _ => {}
        }
    }
}
