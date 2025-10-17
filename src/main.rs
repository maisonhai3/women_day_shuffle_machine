mod context;

use context::ShuffleMachine;

fn main() {
    let shuffle_machine = ShuffleMachine::new();
    let result = shuffle_machine.shuffle();
    println!("Shuffled result: {}", result);
}
