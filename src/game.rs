use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GameState {
  pub tick: u64,
}

pub fn update(starting:GameState) -> GameState {
  let abc = GameState { tick: starting.tick + 1, .. starting };
  abc
}

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}