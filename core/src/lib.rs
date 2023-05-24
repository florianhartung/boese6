use std::path::Path;
use std::sync::Mutex;

use crate::error::Error;
use crate::game::player_decision::DecideFn;
use crate::game::result::{GameResults, MultiGameResults};
use crate::game::Game;
use crate::strategy_loading::LoadedDecideFn;
use rayon::prelude::*;

pub mod error;
pub mod game;
mod strategy_loading;

pub fn play_game_with_functions(
    player_decide_fns: &Vec<&DecideFn>,
    winning_score: i32,
) -> GameResults {
    let game = Game::new(player_decide_fns, winning_score);
    let results = game.play();

    results
}

pub fn play_multiple_games_with_functions(
    player_decide_fns: &Vec<&DecideFn>,
    winning_score: i32,
    num_of_games: usize,
) -> MultiGameResults {
    let multi_results = Mutex::new(MultiGameResults::new(player_decide_fns.len()));

    (0..num_of_games).into_par_iter().for_each(|_| {
        let game = Game::new(player_decide_fns, winning_score);
        let results = game.play();

        multi_results.lock().unwrap().add_game_results(results);
    });

    multi_results.into_inner().unwrap()
}

pub fn play_game(
    player_strategy_files: &[&Path],
    winning_score: i32,
) -> Result<GameResults, Error> {
    let loaded_fns: Vec<LoadedDecideFn> = load_player_strategies(player_strategy_files)?;
    let decide_fns: Vec<&DecideFn> = loaded_fns
        .iter()
        .map(|loaded_fn| loaded_fn.to_decide_fn())
        .collect();

    let results = play_game_with_functions(&decide_fns, winning_score);

    Ok(results)
}

pub fn play_multiple_games(
    player_strategy_files: &[&Path],
    winning_score: i32,
    num_of_games: usize,
) -> Result<MultiGameResults, Error> {
    let loaded_fns: Vec<LoadedDecideFn> = load_player_strategies(player_strategy_files)?;
    let decide_fns: Vec<&DecideFn> = loaded_fns
        .iter()
        .map(|loaded_fn| loaded_fn.to_decide_fn())
        .collect();

    let multi_results =
        play_multiple_games_with_functions(&decide_fns, winning_score, num_of_games);

    Ok(multi_results)
}

fn load_player_strategies(player_strategy_files: &[&Path]) -> Result<Vec<LoadedDecideFn>, Error> {
    player_strategy_files
        .iter()
        .map(|&strategy_file| strategy_loading::load_strategy(strategy_file))
        .collect()
}
