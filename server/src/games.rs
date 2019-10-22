use actix::prelude::*;
use crate::commands::{Message, *};
use crate::IuroError;
use std::collections::HashMap;
use rand::{prelude::*, distributions::Standard};

const EXPECTED_USERS: usize = 4;

pub struct RoomSlot {
    pub recipient: Recipient<Message>,
    pub wins: usize,
}

pub struct Room {
    pub sessions: HashMap<usize, RoomSlot>,
    games: Vec<Games>,
    game: Option<GameInputs>,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            sessions: HashMap::default(),
            game: None,
            games: (0..10).map(|_| random()).collect(),
        }
    }
}

impl Room {
    pub fn join(&mut self, id: usize, recipient: Recipient<Message>) -> Option<Games> {
        self.sessions.insert(id, RoomSlot { recipient, wins: 0 }); 

        if self.sessions.len() == EXPECTED_USERS {
            Some(self.start_game())
        } else {
            None
        }
    }

    fn start_game(&mut self) -> Games {
        let game = self.games.pop().unwrap_or_else(random);
        self.game = Some(match game {
            Games::RockPapiuroScissor => GameInputs::RockPapiuroScissor(HashMap::default())
        });

        if self.games.is_empty() {
            self.games = (0..10).map(|_| random()).collect();
        }

        game
    }

    pub fn update(&mut self, user_id: usize, input: GameInput) -> Result<HashMap<usize, usize>, IuroError> {
        match (self.game.as_mut(), input) {
            (Some(GameInputs::RockPapiuroScissor(hashmap)), GameInput::RockPapiuroScissor(val)) => {
                let mut winners = HashMap::default();
                hashmap.insert(user_id, val);

                if hashmap.len() == self.sessions.len() {
                    let points = hashmap.iter().map(|(k, this)| (k, hashmap.values().map(|other| this.beats(*other) as u8).sum::<u8>()));
                    let max = points.clone().map(|(_, v)| v).max().unwrap_or(0);
                    for (id, _) in points.filter(|(_, points)| *points >= max) {
                        let slot = self.sessions.get_mut(id).ok_or(IuroError::AddrNotFound(*id))?;
                        slot.wins += 1;
                        winners.insert(*id, slot.wins);
                    }
                    hashmap.clear();
                }
                Ok(winners)
            }
            //(Some(GameInputs::WheresIuro(hashmap)), GameInput::WheresIuro(val)) => hashmap.insert(user_id, val),
            (None, _) => Err(IuroError::NoGameHappening),
            //_ => Err("playing wrong game")?,
        }
    }
}

impl RockPapiuroScissorInput {
    fn beats(self, other: Self) -> bool {
        match (self, other) {
            (Self::Rock, Self::Scissor) => true,
            (Self::Scissor, Self::Papiuro) => true,
            (Self::Papiuro, Self::Rock) => true,
            (Self::Rock, Self::Papiuro) => false,
            (Self::Rock, Self::Rock) => false,
            (Self::Papiuro, Self::Scissor) => false,
            (Self::Papiuro, Self::Papiuro) => false,
            (Self::Scissor, Self::Rock) => false,
            (Self::Scissor, Self::Scissor) => false,
        }
    }
}

impl Distribution<Games> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Games {
        let game = match rng.gen_range(0, 1) {
            _ => Games::RockPapiuroScissor,
            //_ => Games::WheresIuro,
        };

        // This is here so the code breaks whenever new variants are added,
        // since the random generation above will only break at runtime
        match game {
            Games::RockPapiuroScissor => game,
            //Games::WheresIuro => game,
        }
    }
}
