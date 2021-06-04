mod files;

use std::io;
use structopt::StructOpt;
use std::char;
use rand::Rng;
use chrono::Utc;


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

    match opts.subcommands {
        Sub::Assign(opts)  => {
            assign(opts.members);
        }
        Sub::History => {
            history();
        }
    }
}

fn assign(members: String){
    if !validate_input_members(&members) {
        println!("Error: Unexpected input format. Please check your input members format.");
        println!("Usage: $ mp Hiro,Walter,Ian,Gabe");
        return;
    }

    let teams = random_assign_teams(members);

    let mut alphabet:u32 = 65; // 'A'
    let mut results:String = String::from(format!("[{}] ", Utc::now().to_string()));

    for team in teams {
        let mut result = format!("Team {}: ", char::from_u32(alphabet).unwrap());
        for member in team { result.push_str(&(format!("{} ", member))) }

        print!("{}\n", result);
        results.push_str(&result);
        alphabet+=1;
    }
    ask_save_history(&mut results);
}

fn history(){
    let home = std::env::var("HOME").unwrap();
    let history_dir = home + "/.mp";
    let history = files::History::new(&history_dir, "mp_history");

    let mut lines = history.get_all_lines();
    lines.reverse();

    let mut max_num = 10;
    if lines.len() < max_num{
        max_num = lines.len();
    }

    for i in 0..max_num {
        println!("{}", lines[i]);
    }

}

fn ask_save_history(results: &mut String) {

    let home = std::env::var("HOME").unwrap();
    let history_dir = home + "/.mp";
    let history = files::History::new(&history_dir, "mp_history");

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

fn random_assign_teams(members_string: String) -> Vec<Vec<String>>{

    let mut members: Vec<&str> = members_string.split(',').collect();
    let members_num = members.len();

    let team_num = members_num / 2;
    let mut team = vec![vec!["".to_string(); 2]; team_num];

    for i in 0..team_num{
        let num1 = rand::thread_rng().gen_range(0..members_num - (i * 2));
        let member1 = members.remove(num1);
        team[i][0] = member1.to_string();

        let num2 = rand::thread_rng().gen_range(0..members_num - 1 - (i * 2));
        let member2 = members.remove(num2);
        team[i][1] = member2.to_string();
    }

    if members_num % 2 != 0 {
        let num3 = rand::thread_rng().gen_range(0..team_num);
        team[num3].push(members[0].to_string());
    }

    team
}

fn validate_input_members(members_string: &String) -> bool {
    if members_string.chars().last().unwrap() == ',' {
        return false
    }
    true
}

#[test]
fn check_random_assign_teams(){
    let result = random_assign_teams("hiro,koji".to_string());
    assert!(result == [["hiro","koji"]] || result == [["koji","hiro"]]);
}

#[test]
fn check_validate_input_members(){
    let members_correct = "hiro,koji".to_string();
    let members_incorrect = "hiro,koji,".to_string();
    assert_eq!(validate_input_members(&members_correct), true);
    assert_eq!(validate_input_members(&members_incorrect), false);
}
