use core::result::Result;
use std::fmt::Error;

pub struct Mess<S : Copy> {
    pub starting_state:S, 
    pub option:fn(S) -> Vec<S>
}

impl<S: Copy> Clone for Mess<S> {
    fn clone(&self) -> Self {
        Self { starting_state: self.starting_state, option: self.option }
    }
}

impl<S : Copy> Mess<S> {
    pub fn play(&self, decider:fn(Vec<S>) -> Option<S>, iter:fn(S)) -> S {
        let mut round = Round {
            game:self.clone(),
            decider
        };

        let mut state = self.starting_state;

        loop {
            iter(state);
            let result = round.play(state);
            match result {
                Some(s) => {
                    state = s;
                    continue;
                }
                None => {break;}
            }
        }

        state
    }
}

pub struct Round<S: Copy> {
    pub game:Mess<S>,
    pub decider:fn(Vec<S>) -> Option<S>,
}

impl<S: Copy> Round<S> {
    pub fn play(&mut self, state:S) -> Option<S> {
        let options = (self.game.option)(state);

        if options.is_empty() { return None; }

        (self.decider)(options)
    }
}