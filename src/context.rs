struct RandomState { }
struct AlphabetState { }
#[derive(Copy, Clone)]
enum Mode {
    RandomState,
    AlphabetState,
}

trait ShuffleBehavior {
    fn shuffle(&self) -> &str;
}

impl ShuffleBehavior for RandomState {
    fn shuffle(&self) -> &str{
        "Hai, Nam, Dung"
    }
}

impl ShuffleBehavior for AlphabetState {
    fn shuffle(&self) -> &str{
        "Hai, Nam, Dung"
    }
}


struct ShuffleMachine {
    random_state: RandomState,
    alphabet_state: AlphabetState,
    current_mode: Mode,
}

impl ShuffleMachine {
    fn new() -> Self{
        ShuffleMachine {
            random_state: RandomState {},
            alphabet_state: AlphabetState {},
            current_mode: Mode::RandomState,
        }
    }

    fn shuffle(&self) -> &str {
        match self.current_mode {
            Mode::RandomState => self.random_state.shuffle(),
            Mode::AlphabetState => self.alphabet_state.shuffle(),
        }
    }
}


