use rand::Rng;

pub fn random_assign_teams(members_string: String) -> Vec<Vec<String>>{

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

pub fn validate_input_members(members_string: &String) -> bool {
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
