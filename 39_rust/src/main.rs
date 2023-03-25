use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let favorite_team: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let games_played: usize = input.trim().parse().unwrap();

    let mut game_results = Vec::new();
    for _ in 0..games_played {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let result: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        game_results.push((result[0], result[1], result[2], result[3]));
    }

    let winning_scenarios = find_winning_scenarios(favorite_team, game_results);
    println!("{}", winning_scenarios);
}

fn find_winning_scenarios(favorite_team: usize, game_results: Vec<(usize, usize, usize, usize)>) -> usize {
    let mut points = vec![0; 4];
    for result in &game_results {
        let (team_a, team_b, score_a, score_b) = result;
        if score_a > score_b {
            points[team_a - 1] += 3;
        } else if score_a < score_b {
            points[team_b - 1] += 3;
        } else {
            points[team_a - 1] += 1;
            points[team_b - 1] += 1;
        }
    }

    let remaining_games = (1..=4)
        .flat_map(|team_a| (team_a + 1..=4).map(move |team_b| (team_a, team_b)))
        .filter(|&(team_a, team_b)| {
            game_results
                .iter()
                .all(|&(played_a, played_b, _, _)| played_a != team_a || played_b != team_b)
        })
        .collect::<Vec<_>>();

    explore(favorite_team, points, remaining_games, 0)
}

fn explore(
    favorite_team: usize,
    mut points: Vec<usize>,
    remaining_games: Vec<(usize, usize)>,
    index: usize,
) -> usize {
    if index == remaining_games.len() {
        let fav_team_points = points[favorite_team as usize - 1];
        return if points.into_iter().all(|p| p <= fav_team_points) { 1 } else { 0 };
    }

    let (team_a, team_b) = remaining_games[index];
    let mut winning_scenarios = 0;

    // Team A wins
    points[team_a as usize - 1] += 3;
    winning_scenarios += explore(favorite_team, points.clone(), remaining_games.clone(), index + 1);
    points[team_a as usize - 1] -= 3;

    // Team B wins
    points[team_b as usize - 1] += 3;
    winning_scenarios += explore(favorite_team, points.clone(), remaining_games.clone(), index + 1);
    points[team_b as usize - 1] -= 3;

    // Draw
    points[team_a as usize - 1] += 1;
    points[team_b as usize - 1] += 1;
    winning_scenarios += explore(favorite_team, points.clone(), remaining_games.clone(), index + 1);

    winning_scenarios
}

