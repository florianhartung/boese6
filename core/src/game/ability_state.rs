#[derive(Debug)]
pub struct AbilityState {
    abilities: [i32; 4],
}

impl AbilityState {
    pub fn new(initial_state: [i32; 4]) -> Self {
        Self {
            abilities: initial_state,
        }
    }

    pub fn d(&mut self) -> &mut i32 {
        &mut self.abilities[0]
    }

    pub fn h(&mut self) -> &mut i32 {
        &mut self.abilities[1]
    }

    pub fn b(&mut self) -> &mut i32 {
        &mut self.abilities[2]
    }

    pub fn w(&mut self) -> &mut i32 {
        &mut self.abilities[3]
    }
}
