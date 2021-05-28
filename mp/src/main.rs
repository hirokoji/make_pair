use structopt::StructOpt;
use rand::Rng;
// use rand::Rng;

#[derive(StructOpt)]
struct Cli {
    members: String
}

fn main() {
    let args = Cli::from_args();
    let result = random_assign(args.members);
    println!("{}", result);

}

fn random_assign(members_string: String) -> String{

    let members: Vec<&str> = members_string.split(',').collect();
    let num = rand::thread_rng().gen_range(0..members.len());

    members[num].to_string()
}


#[test]
fn check_random_assign(){
    let result = random_assign("hiro,koji".to_string());
    assert!(result == "hiro".to_string() || result == "koji".to_string());
}


