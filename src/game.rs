

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GameState {
  pub tick: u64,
}

pub fn update(starting:GameState) -> GameState {
  let abc = GameState { tick: starting.tick + 1, .. starting };
  abc
}
