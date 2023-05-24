use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use std::time::SystemTime;

use b6_core::game::player_decision::{DecideFn, PlayerStrategy};
use b6_core::play_multiple_games_with_functions;

use crate::strategies::{Player1, Player2};

mod strategies;

const NUM_GAMES: usize = 10_000_000;

fn main() {
    let players = vec![
        &(Player1::decide as DecideFn),
        &(Player1::decide as DecideFn),
        &(Player1::decide as DecideFn),
        &(Player2::decide as DecideFn),
        &(Player2::decide as DecideFn),
        &(Player2::decide as DecideFn),
    ];

    let before = SystemTime::now();

    let results = play_multiple_games_with_functions(&players, 223, NUM_GAMES);

    let elapsed_millis = SystemTime::now()
        .duration_since(before)
        .unwrap()
        .as_millis();
    let games_per_second = NUM_GAMES as u128 * 1000 / elapsed_millis;

    println!(
        "Playing games took {}ms. That's {} games per second",
        elapsed_millis,
        games_per_second.to_formatted_string(&Locale::de)
    );

    results
        .player_won_games
        .iter()
        .enumerate()
        .sorted_by_key(|&(_id, &num_wins)| num_wins)
        .rev()
        .enumerate()
        .for_each(|(position, (id, &num_wins))| {
            println!("{}.  Player {} ({} wins)", position + 1, id, num_wins)
        });
}
