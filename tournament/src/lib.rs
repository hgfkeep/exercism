use std::collections::{BTreeMap, BTreeSet, HashMap};

//比赛次数
#[derive(Debug)]
struct Team {
    win: i32,
    loss: i32,
    draw: i32,
}

impl Team {
    fn new() -> Self {
        Team {win: 0, loss: 0, draw: 0}
    }
}

///
/// 1. 解析team -> score;
/// 2. 排序构造BTreeMap：
///     score -> BTreeSet<name>
/// 3. 统计输出
///
pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();
    match_results.split("\n").for_each(|s| {
        println!("\nprocess-----------------\ninput: {}", s);
        if s.len() > 0 {
            let bat: Vec<&str> = s.split(';').collect();
            bat_parser(&mut teams, bat);
        }
    });

    println!("all teams: {:?}", teams);

    let mut matches: BTreeMap<i32, BTreeSet<String>> = BTreeMap::new();
    teams.iter().for_each(|(name, team)| {
        let score = team.win * 3 + team.draw;
        matches
            .entry(score)
            .or_insert(BTreeSet::new())
            .insert(name.clone());
    });

    println!("sorted: {:?}", matches);

    let mut res: String = String::new();
    res.push_str("Team                           | MP |  W |  D |  L |  P");
    matches.iter().rev().for_each(|(score, names)| {
        if names.len() > 0 {
            names.iter().for_each(|name| {
                if let Some(team) = teams.get(name) {
                    res.push('\n');
                    let mp = team.win + team.loss + team.draw;
                    res.extend(
                        format!(
                            "{:<31}|  {} |  {} |  {} |  {} |{:>3}",
                            name, mp, team.win, team.draw, team.loss, score
                        )
                        .chars(),
                    );
                }
            });
        }
    });
    res
}

//解析每行输入
fn bat_parser(teams: &mut HashMap<String, Team>, bat: Vec<&str>) {
    println!("parser {:?}", bat);
    assert_eq!(3, bat.len());
    let mut win: String = String::new();
    let mut los: String = String::new();
    match bat[2] {
        "win" => {
            win.push_str(bat[0]);
            los.push_str(bat[1]);
        }
        "loss" => {
            win.push_str(bat[1]);
            los.push_str(bat[0]);
        }
        _ => {}
    }

    if win.len() > 0 {
        let mut t = teams.entry(win).or_insert(Team::new());
        t.win += 1;
        let mut t = teams.entry(los).or_insert(Team::new());
        t.loss += 1;
    } else {
        let mut t = teams.entry(bat[0].to_string()).or_insert(Team::new());
        t.draw += 1;
        let mut t = teams.entry(bat[1].to_string()).or_insert(Team::new());
        t.draw += 1;
    }
}
