#[derive(Debug, PartialEq, Clone)]
enum State {
    NoChange,
    Odd,
    Even,
}

pub fn parity(input: &str) ->bool {
    use State::*;

    let mut state = NoChange;

    for ch in input.chars() {
        state = match (state.clone(), ch) {
            (NoChange, '0') => NoChange,  // Stay in NoChange for '0'
            (NoChange, '1') => Odd,       // Transition to Odd on '1'
            (Odd, '0') => Odd,            // Stay in Odd for '0'
            (Odd, '1') => Even,           // Transition to Even on '1'
            (Even, '0') => Even,          // Stay in Even for '0'
            (Even, '1') => Odd,           // Transition back to Odd on '1'
            _ => state,  // Handle unexpected input by staying in the same state
        };

        match state {
            Odd => println!("ODD"),
            Even => println!("EVEN"),
            NoChange => {}  // No output in NoChange state
        }
    }
    state == Even
}
