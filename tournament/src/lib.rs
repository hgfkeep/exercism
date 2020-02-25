use std::cmp::Ordering;
use std::collections::HashMap;
//比赛次数
#[derive(Debug)]
struct Team {
    name: String,
    win: i32,
    loss: i32,
    draw: i32,
}

impl Team {
    fn new(name: String) -> Self {
        Team {
            name: name,
            win: 0,
            loss: 0,
            draw: 0,
        }
    }
    fn points(&self) -> i32 {
        self.win * 3 + self.draw
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();

    //处理每行输入
    match_results.split("\n").for_each(|s| {
        if s.len() > 0 {
            let bat: Vec<&str> = s.split(';').collect();
            bat_parser(&mut teams, bat);
        }
    });

    //排序
    let mut sorted_teams: Vec<&Team> = teams.values().collect();
    sorted_teams.sort_by(|a, b| match a.points().cmp(&b.points()).reverse() {
        Ordering::Equal => a.name.cmp(&b.name),
        other => other,
    });

    //输出
    let mut res: String = String::new();
    res.push_str("Team                           | MP |  W |  D |  L |  P");
    sorted_teams.iter().for_each(|team| {
        res.push('\n');
        let mp = team.win + team.loss + team.draw;
        res.extend(
            format!(
                "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                team.name,
                mp,
                team.win,
                team.draw,
                team.loss,
                team.points()
            )
            .chars(),
        );
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
        let mut t = teams.entry(win.clone()).or_insert(Team::new(win));
        t.win += 1;
        let mut t = teams.entry(los.clone()).or_insert(Team::new(los));
        t.loss += 1;
    } else {
        let mut t = teams
            .entry(bat[0].to_string())
            .or_insert(Team::new(bat[0].to_string()));
        t.draw += 1;
        let mut t = teams
            .entry(bat[1].to_string())
            .or_insert(Team::new(bat[1].to_string()));
        t.draw += 1;
    }
}
