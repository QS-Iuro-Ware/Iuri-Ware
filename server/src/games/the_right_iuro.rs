use crate::prelude::*;
use std::collections::HashMap;

pub struct Update<'a> {
    pub user_id: usize,
    pub input: Vec<u8>,
    pub state: &'a mut (Vec<u8>, HashMap<usize, Vec<u8>>),
}

impl<'a> Update<'a> {
    pub fn consume(self, sessions: &mut HashMap<usize, RoomSlot>) -> Result<bool, IuroError> {
        let mut ret = false;
        let Update {
            state,
            user_id,
            input,
        } = self;
        state.1.insert(user_id, input);

        // All users answered
        if state.1.len() == sessions.len() {
            // Computes points for each user
            let points = state.1.iter().map(|(id, input)| {
                (id, (input == &state.0) as u8)
            });

            // Selects only winning users
            for (id, _) in points.filter(|(_, points)| *points > 0) {
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
