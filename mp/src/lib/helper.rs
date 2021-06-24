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

pub fn has_same_team(teams_a: &Vec<Vec<&str>>, teams_b: &Vec<Vec<&str>>) -> bool {

    let big_teams;
    let small_teams;

    if teams_a.len() > teams_b.len(){
        big_teams = teams_a;
        small_teams = teams_b;
    }else{
        big_teams = teams_b;
        small_teams = teams_a;
    }

    for team_a in big_teams {
        for team_b in small_teams {
            if team_a.contains(&team_b[0]) && team_a.contains(&team_b[1]) {
                return true;
            }
        }
    }

    false
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
fn test_has_same_team(){
    let teams_a = vec![vec!["ajay", "hiro"], vec!["konark", "yong"], vec!["aman", "anandita", "philp"]];
    let teams_b = vec![vec!["ajay", "hiro"], vec!["konark", "yong"], vec!["aman", "anandita", "philp"]];
    assert_eq!(has_same_team(&teams_a, &teams_b), true);

    let teams_a = vec![vec!["ajay", "hiro"], vec!["konark", "yong"], vec!["aman", "anandita", "philp"]];
    let teams_b = vec![vec!["ajay", "philip"], vec!["konark", "hiro"], vec!["aman", "anandita", "yong"]];
    assert_eq!(has_same_team(&teams_a, &teams_b), true);

    let teams_a = vec![vec!["ajay", "hiro"], vec!["konark", "yong"], vec!["aman", "anandita", "philp"]];
    let teams_b = vec![vec!["ajay", "yong"], vec!["konark", "anandita"], vec!["aman", "hiro", "philp"]];
    assert_eq!(has_same_team(&teams_a, &teams_b), false);

    let teams_a = vec![vec!["hiro", "yong"], vec!["aman", "philip"], vec!["konark", "yong", "philp"]];
    let teams_b = vec![vec!["ajay", "hiro"], vec!["hiro", "yong"] ];
    assert_eq!(has_same_team(&teams_a, &teams_b), true);

    let teams_a = vec![vec!["ajay", "hiro"], vec!["hiro", "yong"] ];
    let teams_b = vec![vec!["hiro", "yong"], vec!["aman", "philip"], vec!["konark", "yong", "philp"]];
    assert_eq!(has_same_team(&teams_a, &teams_b), true);

    let teams_a = vec![vec!["ajay", "hiro"], vec!["hiro", "konark"] ];
    let teams_b = vec![vec!["hiro", "yong"], vec!["aman", "philip"], vec!["konark", "yong", "philp"]];
    assert_eq!(has_same_team(&teams_a, &teams_b), false);

    let teams_a = vec![vec!["hiro", "yong"], vec!["aman", "philip"], vec!["konark", "yong", "philp"]];
    let teams_b = vec![vec!["ajay", "hiro"], vec!["hiro", "konark"] ];
    assert_eq!(has_same_team(&teams_a, &teams_b), false);
}
