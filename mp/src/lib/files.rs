use std::path::{Path,PathBuf};
use std::ops::Not;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::writeln;
use regex::Regex;

pub struct History{
    dir_path: PathBuf,
    file_path: PathBuf,
}

impl History {
    pub fn new(dir_path: &str, filename: &str) -> Self{
        History{ dir_path: Path::new(dir_path).to_path_buf(), file_path: Path::new(dir_path).join(filename) }
    }

    pub fn save(&self, result:&str){

       if self.dir_path.exists().not() {
            create_dir(&self.dir_path).expect(&format!("Can't create dir ({}) to store history", self.dir_path.display()));
        }
        if self.file_path.exists().not() {
            File::create(&self.file_path).expect(&format!("Can't create file ({}) to store history", self.file_path.display()));
        }
        let mut file = OpenOptions::new().write(true).append(true).open(&self.file_path).unwrap();
        writeln!(file,"{}", result).unwrap();

    }

    pub fn get_all_lines(&self) -> Vec<String>{

        let file = OpenOptions::new().read(true).append(true).open(&self.file_path).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<_>= reader.lines().map(|line| {line.unwrap()}).collect();
        lines
    }

    pub fn get_last_team(&self) -> Vec<Vec<String>> {
        let last = self.get_last_line();

        let re = Regex::new(r"Team .: ").unwrap();
        let team_string = last.split("] ").last().unwrap();
        let team :Vec<Vec<_>> = Regex::split(&re, team_string).filter(|&team| team != "").map(|team|team.trim().split(' ').map(String::from).collect()).collect();
        team
    }

    fn get_last_line(&self) -> String{

        let lines = self.get_all_lines();
        let last = lines.last().unwrap().to_string();
        last

    }
}

#[test]
fn test_history_io(){

    let pwd = std::env::var("PWD").unwrap();
    let history = History::new(&pwd, "test.txt");

    println!("saved to {}", history.file_path.display());
    let keyword = "[2021-06-02 04:05:47.029899 UTC] Team A: ajay hiro Team B: konark yong Team C: aman philp anandita";
    history.save(keyword);
    assert_eq!(keyword, history.get_last_line());
}

#[test]
fn test_get_last_team(){
    let pwd = std::env::var("PWD").unwrap();
    let history = History::new(&pwd, "test.txt");
    let team = history.get_last_team();
    let expected_result = vec![vec!["ajay", "hiro"], vec!["konark", "yong"], vec!["aman", "philp", "anandita"]];
    assert_eq!(team, expected_result);
}