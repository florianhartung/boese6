use crate::game::ability_state::AbilityState;
use crate::game::player_decision::{DecideFn, Decision};
use crate::game::rng::throw_dice;

pub struct RoundLogicHandler<'a> {
    remaining_abilities: &'a mut AbilityState,
    decide_fn: &'a DecideFn,
    total_score: i32,

    num_throws: i32,
    round_score: i32,
    last_score_if_h: Option<i32>,
}

impl<'a> RoundLogicHandler<'a> {
    pub fn new(
        remaining_abilities: &'a mut AbilityState,
        total_score: i32,
        decide_fn: &'a DecideFn,
    ) -> Self {
        Self {
            remaining_abilities,
            num_throws: 0,
            round_score: 0,
            last_score_if_h: None,
            total_score,
            decide_fn,
        }
    }

    pub fn play_throw(&mut self) -> bool {
        self.num_throws += 1;

        let w1 = throw_dice();
        let w2 = throw_dice();

        let is_b6 = w1 == 6 || w2 == 6 || w1 + w2 == 6;

        if !self.pre_decision(w1, w2, is_b6) {
            return false; // END ROUND
        }

        let decision: Decision = (self.decide_fn)(
            w1,
            w2,
            self.round_score,
            self.total_score,
            self.num_throws,
            *self.remaining_abilities.d(),
            *self.remaining_abilities.h(),
            *self.remaining_abilities.b(),
            *self.remaining_abilities.w(),
        )
        .try_into()
        .unwrap();

        self.post_decision(w1, w2, is_b6, decision)
    }

    // Handels the pre decision logic, returns whether the round should continue
    fn pre_decision(&mut self, _w1: i32, _w2: i32, is_b6: bool) -> bool {
        if is_b6 {
            if let Some(last_throw_score) = self.last_score_if_h {
                self.round_score -= last_throw_score / 2;
                self.last_score_if_h = None;
                return false; // END ROUND
            }
        }
        self.last_score_if_h = None;
        return true;
    }

    // Handels the post decision logic, returns whether the round should continue
    fn post_decision(&mut self, w1: i32, w2: i32, is_b6: bool, decision: Decision) -> bool {
        match decision {
            Decision::End => {
                if is_b6 {
                    self.round_score = 0;
                } else {
                    self.round_score += w1 + w2;
                }
                false
            }
            Decision::Continue => {
                if is_b6 {
                    self.round_score = 0;
                    false
                } else {
                    self.round_score += w1 + w2;
                    true
                }
            }
            Decision::H => {
                Self::use_ability(self.remaining_abilities.h(), 'H');

                if is_b6 {
                    self.round_score = 0;
                    false
                } else {
                    self.round_score += w1 + w2;
                    self.last_score_if_h = Some(w1 + w2);
                    true
                }
            }
            Decision::B => {
                Self::use_ability(self.remaining_abilities.b(), 'B');

                if !is_b6 {
                    self.round_score += w1 + w2;
                }
                false
            }
            Decision::W => {
                Self::use_ability(self.remaining_abilities.w(), 'W');

                self.round_score += w1 + w2;
                false
            }
            Decision::D => {
                Self::use_ability(self.remaining_abilities.d(), 'D');
                assert!(!(is_b6 && !(w1 + w2 == 12)), "A player tried to use the D ability even though it is a b6, with w1 + w2 != 12");

                self.round_score += 2 * (w1 + w2);
                false
            }
        }
    }

    fn use_ability(ability_count: &mut i32, ability_name: char) {
        assert!(
            *ability_count >= 1,
            "A player tried to use the ability {} even though they don't have any left.",
            ability_name,
        );
        *ability_count -= 1;
    }

    pub fn into_final_score(self) -> i32 {
        self.round_score
    }
}
