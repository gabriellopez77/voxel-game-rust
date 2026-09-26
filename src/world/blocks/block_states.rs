use strum_macros::EnumIter;


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Facing { Up, Down, North, South, East, West }

impl Default for Facing { fn default() -> Self { Self::North } }


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Axis { X, Y, Z }

impl Default for Axis { fn default() -> Self { Self::X } }

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum BlockStatesTypes {
    Facing(Facing),
    Axis(Axis),
}

pub struct BlockStates {
    states: Vec<BlockStatesTypes>,
}

impl BlockStates {
    pub fn new(states: Vec<BlockStatesTypes>) -> Self {
        Self { states }
    }

    /// check if states contains required_states
    pub fn has(&self, required_states: &[BlockStatesTypes]) -> bool {
        required_states.iter().all(|s| self.states.contains(s))
    }

    pub fn has_state(&self, state: BlockStatesTypes) -> bool {
        self.states.contains(&state)
    }

    pub fn get_state(&self, state: BlockStatesTypes) -> Option<BlockStatesTypes> {
        if let Some(idx) = self.states.iter().position(|s| *s == state) {
            return Some(self.states[idx]);
        }

        return None;
    }

    pub fn get_discriminant(&self, state: BlockStatesTypes) -> Option<BlockStatesTypes> {
        if let Some(idx) = self.states.iter().position(|s| std::mem::discriminant(s) == std::mem::discriminant(&state)) {
            return Some(self.states[idx]);
        }

        return None;
    }
}
