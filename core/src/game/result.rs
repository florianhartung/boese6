pub struct GameResults {
    pub player_scores: Vec<i32>,
    pub winning_score: i32,
}

impl GameResults {
    pub fn new(score_by_player_id: Vec<i32>, winning_score: i32) -> Self {
        Self {
            player_scores: score_by_player_id,
            winning_score,
        }
    }

    pub fn get_winner_ids(&self) -> Vec<usize> {
        self.player_scores
            .iter()
            .enumerate()
            .filter_map(|(id, &score)| (score >= self.winning_score).then_some(id))
            .collect()
    }
}

pub struct MultiGameResults {
    pub player_won_games: Vec<usize>,
}

impl MultiGameResults {
    pub fn new(num_players: usize) -> Self {
        Self {
            player_won_games: vec![0; num_players],
        }
    }

    pub fn add_game_results(&mut self, game_results: GameResults) {
        // Add one win for every winner in `won_games_by_player_ids`
        game_results
            .get_winner_ids()
            .into_iter()
            .for_each(|player_id| self.player_won_games[player_id] += 1);
    }
}
