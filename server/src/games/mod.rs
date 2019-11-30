use actix::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub mod rock_papiuro_scissor;
pub mod the_right_iuro;

pub mod prelude {
    pub use super::{GameInput, GameState, Game};
    pub use super::rock_papiuro_scissor::RockPapiuroScissorInput;
}

use crate::prelude::*;

/// Game input sent from user
#[derive(Deserialize, Message, Debug)]
pub enum GameInput {
    RockPapiuroScissor(RockPapiuroScissorInput),
    TheRightIuro(Vec<u8>),
}

/// Enumerates available games
#[derive(Serialize, Clone, Debug)]
pub enum Game {
    /// Rock Paper Scissors Iuro's version
    RockPapiuroScissor,
    TheRightIuro(Vec<u8>)
}


/// Each game holds its own state
#[derive(Debug)]
pub enum GameState {
    RockPapiuroScissor(HashMap<usize, RockPapiuroScissorInput>),
    TheRightIuro((Vec<u8>, HashMap<usize, Vec<u8>>)),
}

