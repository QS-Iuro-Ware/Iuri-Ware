use actix::prelude::*;
use rand::{distributions::Standard, prelude::*};
use std::collections::HashMap;

use crate::{games, prelude::*};

const EXPECTED_USERS: usize = 4;

/// User's data when inside of a room
pub struct RoomSlot {
    pub recipient: Recipient<Broadcast>,
    pub name: String,
    pub wins: usize,
}

/// Manages room's users and its games
pub struct Room {
    sessions: HashMap<usize, RoomSlot>,
    games: Vec<Game>,
    game: Option<GameState>,
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
    /// Inserts user in room if there is space, broadcasts next game if join triggered it
    pub fn join(&mut self, id: usize, slot: RoomSlot) -> JoinResult {
        if self.sessions.len() == EXPECTED_USERS {
            return JoinResult::Full;
        }

        self.sessions.insert(id, slot);

        if self.sessions.len() == EXPECTED_USERS {
            JoinResult::NewGame(self.start_game())
        } else {
            debug!("User {} of {} joined", self.sessions.len(), EXPECTED_USERS);
            JoinResult::NoGame
        }
    }

    /// Instantiates next game in queue
    pub fn start_game(&mut self) -> Game {
        let game = self.games.pop().unwrap_or_else(random);

        self.game = Some(match game {
            Game::RockPapiuroScissor => GameState::RockPapiuroScissor(HashMap::default()),
        });

        // This should never happen, but let's handle all branches appropriately
        if self.games.is_empty() {
            error!("Games list is empty when it shouldn't");
            debug_assert!(false, "Games list is empty when it shouldn't");
            self.games = (0..10).map(|_| random()).collect();
        }

        game
    }

    /// Updates game state with user's input
    pub fn update(
        &mut self,
        user_id: usize,
        input: GameInput,
    ) -> Result<HashMap<String, usize>, IuroError> {
        match (self.game.as_mut(), input) {
            (Some(GameState::RockPapiuroScissor(state)), GameInput::RockPapiuroScissor(input)) => {
                let update = games::rock_papiuro_scissor::Update {
                    user_id,
                    input,
                    state,
                };
                if update.consume(&mut self.sessions)? {
                    Ok(self
                        .sessions
                        .values()
                        .map(|slot| (slot.name.clone(), slot.wins))
                        .collect())
                } else {
                    Ok(HashMap::default())
                }
            }
            (None, _) => {
                warn!("User sent game input when it wasn't possible");
                Err(IuroError::NoGameHappening)
            }
        }
    }

    pub fn sessions(&self) -> &HashMap<usize, RoomSlot> {
        &self.sessions
    }

    pub fn sessions_mut(&mut self) -> &mut HashMap<usize, RoomSlot> {
        &mut self.sessions
    }

    pub fn remove_session(&mut self, id: usize) -> Option<RoomSlot> {
        self.sessions.remove(&id)
    }

    pub fn reset_game(&mut self) {
        self.game = None;
    }
}

impl Distribution<Game> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Game {
        let game = match rng.gen_range(0, 1) {
            _ => Game::RockPapiuroScissor,
        };

        // This is here so the code breaks whenever new variants are added,
        // since the random generation above will only break silently at runtime
        match game {
            Game::RockPapiuroScissor => game,
        }
    }
}

/// Possible results of trying to join a room
pub enum JoinResult {
    /// User joining triggered game start
    NewGame(Game),
    /// User joined, but game is not ready to start yet
    NoGame,
    /// Can't join room since it's already full
    Full,
}
