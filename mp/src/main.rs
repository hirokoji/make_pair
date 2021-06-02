use structopt::StructOpt;
use std::char;
use rand::Rng;
use chrono::Utc;

use std::io;
use std::path::Path;
use std::ops::Not;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::writeln;

#[derive(StructOpt)]
struct Cli {
    members: String
}

fn main() {
    let args = Cli::from_args();

    if !validate_input_members(&args.members) {
        println!("Error: Unexpected input format. Please check your input members format.");
        println!("Usage: $ mp Hiro,Walter,Ian,Gabe");
        return;
    }

    let teams = random_assign_teams(args.members);

    let mut alphabet:u32 = 65; // 'A'
    let mut history:String = String::from(format!("[{}] " ,Utc::now().to_string()));

    for team in teams {
        let mut output = format!("Team {}: ",char::from_u32(alphabet).unwrap());
        for member in team { output.push_str(&(format!("{} ", member))) }

        print!("{}\n", output);
        history.push_str(&output);
        alphabet+=1;
    }

    ask_save_history(&mut history)
}

fn ask_save_history(history: &mut String) {
    loop {
        println!("\nDo you save the result to history? [y/n]: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "y" | "yes" => {
                save_history(&history);
                println!("saved the result");
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

fn save_history(result:&str){

    let dir_name = ".mp";
    let file_name = "mp_history";
    let home = std::env::var("HOME").unwrap();
    let home_path = Path::new(&home);

    let pj_path = home_path.join(dir_name);
    if pj_path.exists().not() {
        create_dir(&pj_path).expect(&format!("Can't create dir ({}) to store history", pj_path.display()));
    }

    let history_path = pj_path.join(file_name);
    if history_path.exists().not() {
        File::create(&history_path).expect(&format!("Can't create file ({}) to store history", history_path.display()));
    }

    let mut file = OpenOptions::new().write(true).append(true).open(history_path).unwrap();
    writeln!(file,"{}", result).unwrap();

}

fn get_history() -> String{

    let dir_name = ".mp";
    let file_name = "mp_history";
    let home = std::env::var("HOME").unwrap();
    let home_path = Path::new(&home);
    let history_path = home_path.join(dir_name).join(file_name);

    let file = OpenOptions::new().read(true).append(true).open(history_path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<_>= reader.lines().map(|line| {line.unwrap()}).collect();
    let last = lines.last().unwrap().to_string();

    last
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

#[test]
fn check_save_history(){
    let keyword = "This line added from unit Test";
    save_history(keyword);
    assert_eq!(get_history(), keyword.to_string())
}