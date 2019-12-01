use crate::prelude::*;
use actix::prelude::*;
use serde::{Deserialize};
use std::collections::HashMap;

/// Input options for `RockPapiuroScissor`
#[derive(Deserialize, Copy, Clone, Message, Debug)]
pub enum RockPapiuroScissorInput {
    Rock,
    Papiuro,
    Scissor,
}

impl RockPapiuroScissorInput {
    /// Returns if user wins against other
    pub fn beats(self, other: Self) -> bool {
        match (self, other) {
            (Self::Rock, Self::Scissor) => true,
            (Self::Scissor, Self::Papiuro) => true,
            (Self::Papiuro, Self::Rock) => true,
            _ => false,
        }
    }
}

pub struct Update<'a> {
    pub user_id: usize,
    pub input: RockPapiuroScissorInput,
    pub state: &'a mut HashMap<usize, RockPapiuroScissorInput>,
}

impl<'a> Update<'a> {
    pub fn consume(self, sessions: &mut HashMap<usize, RoomSlot>) -> Result<bool, IuroError> {
        let mut ret = false;
        let Update {
            state,
            user_id,
            input,
        } = self;
        state.insert(user_id, input);

        // All users answered
        if state.len() == sessions.len() {
            // Computes points for each user
            let points = state.iter().map(|(id, this_input)| {
                let user_points: u8 = state
                    .values()
                    // Beating a user gives 1 point
                    .map(|other_input| this_input.beats(*other_input) as u8)
                    .sum();
                (id, user_points)
            });

            // Get winning threshold (more than one user can win)
            let max = points
                .clone()
                .map(|(_id, points)| points)
                .max()
                .unwrap_or(1);

            // Selects only winning users
            for (id, _) in points.filter(|(_, points)| *points >= max) {
                let slot = sessions
                    .get_mut(id)
                    // This should never happen
                    .ok_or(IuroError::AddrNotFound(*id))?;

                slot.wins += 1;
            }
            ret = true;
        }
        Ok(ret)
    }
}
