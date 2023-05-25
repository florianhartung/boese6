use crate::game::ability_state::AbilityState;
use crate::game::player_decision::DecideFn;
use crate::game::rng::Rng;
use crate::game::round_logic_handler::RoundLogicHandler;

pub struct Player<'a> {
    pub id: usize,
    decide: &'a DecideFn,
    pub total_score: i32,
    remaining_abilities: AbilityState,
}

impl<'a> Player<'a> {
    pub fn new(id: usize, decide: &'a DecideFn) -> Self {
        Self {
            id,
            decide,
            total_score: 0,
            remaining_abilities: AbilityState::new([1, 5, 3, 2]),
        }
    }

    pub fn play_round(&mut self, rng: &mut Rng) {
        let mut round_logic_handler = RoundLogicHandler::new(
            &mut self.remaining_abilities,
            self.total_score,
            &self.decide,
            rng,
        );

        while round_logic_handler.play_throw() {}

        self.total_score += round_logic_handler.into_final_score();
    }
}
