use b6_core::game::player_decision::{Decision, PlayerStrategy};

pub struct Player1;

pub struct Player2;

impl PlayerStrategy for Player1 {
    extern "C" fn decide(
        _w1: i32,
        _w2: i32,
        _round_score: i32,
        _total_score: i32,
        _num_throws: i32,
        _d: i32,
        _h: i32,
        _b: i32,
        _w: i32,
    ) -> i32 {
        Decision::Continue.into()
    }
}

impl PlayerStrategy for Player2 {
    extern "C" fn decide(
        _w1: i32,
        _w2: i32,
        _round_score: i32,
        _total_score: i32,
        num_throws: i32,
        _d: i32,
        _h: i32,
        _b: i32,
        _w: i32,
    ) -> i32 {
        if num_throws >= 2 {
            Decision::End
        } else {
            Decision::Continue
        }
        .into()
    }
}
