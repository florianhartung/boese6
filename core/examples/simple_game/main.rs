use b6_core::game::player_decision::{DecideFn, PlayerStrategy};
use b6_core::play_game_with_functions;

use crate::strategies::{Player1, Player2};

mod strategies;

fn main() {
    let players = vec![
        &(Player1::decide as DecideFn),
        &(Player2::decide as DecideFn),
    ];

    let results = play_game_with_functions(&players, 100);

    results
        .player_scores
        .iter()
        .enumerate()
        .for_each(|(player_id, &player_score)| println!("Player {player_id}: {player_score}"));
}
