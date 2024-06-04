

pub struct Mess<S : Clone> {
    pub starting_state:S, 
    pub option:fn(S) -> Vec<S>
}

impl<S: Clone> Clone for Mess<S> {
    fn clone(&self) -> Self {
        Self { starting_state: self.starting_state.clone(), option: self.option.clone() }
    }
}

impl<S : Clone> Mess<S> {
    pub fn play(&self, decider:fn(Vec<S>) -> Option<S>, iter:fn(S)) -> S {
        let mut round = Round {
            game:self.clone(),
            decider
        };

        let mut state = self.starting_state.clone();

        loop {
            iter(state.clone());
            let result = round.play(state.clone());
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

pub struct Round<S: Clone> {
    pub game:Mess<S>,
    pub decider:fn(Vec<S>) -> Option<S>,
}

impl<S: Clone> Round<S> {
    pub fn play(&mut self, state:S) -> Option<S> {
        let options = (self.game.option)(state);

        if options.is_empty() { return None; }

        (self.decider)(options)
    }
}