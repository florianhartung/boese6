use crate::game::player::Player;
use crate::game::player_decision::DecideFn;
use crate::game::result::GameResults;
use crate::game::rng::Rng;

mod ability_state;
mod player;
pub mod player_decision;
pub mod result;
mod rng;
mod round_logic_handler;

pub struct Game<'a> {
    players: Vec<Player<'a>>,
    winning_score: i32,
}

impl<'a> Game<'a> {
    pub fn new(players: &Vec<&'a DecideFn>, winning_score: i32) -> Self {
        let players = players
            .into_iter()
            .enumerate()
            .map(|(id, &decide_fn)| Player::new(id, decide_fn))
            .collect();

        Self {
            players,
            winning_score,
        }
    }

    /// Plays a game and returns a list of players ids and their respective scores
    pub fn play(mut self) -> GameResults {
        let mut rng = Rng::new();
        let mut finished = false;
        while !finished {
            self.players.iter_mut().for_each(|p| {
                p.play_round(&mut rng);
                if p.total_score >= self.winning_score {
                    finished = true;
                }
            });
        }

        let score_by_player_id = self.players.iter().map(|p| p.total_score).collect();
        GameResults::new(score_by_player_id, self.winning_score)
    }
}
