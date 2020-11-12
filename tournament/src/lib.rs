use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

pub fn tally(match_results: &str) -> String {
    let mut table: Table = Table::new();
    match_results.lines().for_each(|line| {
        let match_result: Vec<&str> = line.split(';').collect();
        let home_team_result = match_result[2];
        table
            .entry(String::from(match_result[0]))
            .or_insert_with(|| TeamResults::new(match_result[0]))
            .add_result(home_team_result, TeamType::Home);
        table
            .entry(String::from(match_result[1]))
            .or_insert_with(|| TeamResults::new(match_result[1]))
            .add_result(home_team_result, TeamType::Away);
    });
    table.to_string()
}

type TeamName = String;
#[derive(Default)]
struct Table<'a> {
    results: HashMap<TeamName, TeamResults<'a>>,
}

impl Display for Table<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TABLE_HEADER: &str = "Team                           | MP |  W |  D |  L |  P";
        let base_table = String::from(TABLE_HEADER);
        let mut table = self
            .results
            .iter()
            .map(|(_, results)| results)
            .collect::<Vec<&TeamResults>>();
        table.sort_by(|a, b| b.points().cmp(&a.points()).then_with(|| a.name.cmp(b.name)));
        let table = table.iter().fold(base_table, |mut table, team_results| {
            table.push_str(&team_results.to_string());
            table
        });
        write!(f, "{}", table)
    }
}

impl<'a> Deref for Table<'a> {
    type Target = HashMap<TeamName, TeamResults<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.results
    }
}

impl DerefMut for Table<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.results
    }
}

impl Table<'_> {
    fn new() -> Self {
        Default::default()
    }
}

enum TeamType {
    Home,
    Away,
}
#[derive(Debug)]
struct TeamResults<'a> {
    name: &'a str,
    wins: usize,
    draws: usize,
    losses: usize,
}

impl<'a> TeamResults<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn matches_played(&self) -> usize {
        self.wins + self.draws + self.losses
    }

    fn points(&self) -> usize {
        self.wins * 3 + self.draws
    }

    fn add_result(&mut self, home_team_result: &str, team_type: TeamType) {
        match (team_type, home_team_result) {
            (TeamType::Home, "win") => {
                self.wins += 1;
            }
            (TeamType::Home, "loss") => {
                self.losses += 1;
            }
            (TeamType::Away, "win") => {
                self.losses += 1;
            }
            (TeamType::Away, "loss") => {
                self.wins += 1;
            }
            (_, "draw") => {
                self.draws += 1;
            }
            (_, _) => unreachable!(),
        }
    }
}

impl Display for TeamResults<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &format!(
                "\n{:31}|{:3} |{:3} |{:3} |{:3} |{:3}",
                self.name,
                self.matches_played(),
                self.wins,
                self.draws,
                self.losses,
                self.points()
            )
        )
    }
}
