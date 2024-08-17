/*
b) Check that there are no more than two consecutive 1’s. That is, accept unless
111 is a substring of the input string read so far.
What is the intuitive meaning of each of your states?
10.2.2: Indicate the sequence of states and the outputs when your automata from
Exercise 10.2.1 are given the input 101001101110.
Input: 101001101110
Output:
ZERO
ONE
ZERO
ONE
ZERO
ZERO
ONE
TWO
ZERO
ONE
TWO
REJECT


void three_conseq_ones()
{
    puts("\n Enter reject 3 conseq bits: ");
    int x;

ZERO:
    x = getchar();
    puts("ZERO");
    if (x == '0') goto ZERO;
    if (x == '1') goto ONE;
    goto finis;
    
ONE:  puts("ONE");
    x = getchar();
    if (x == '0') goto ZERO;
    if (x == '1') goto TWO;
    goto finis;
   
TWO:  puts("TWO");
    x = getchar();
    if (x == '0') goto ZERO;
    if (x == '1') goto REJECT;
    goto finis;
REJECT:
    puts("REJECT");
finis:;
}
*/
#[derive(Debug, PartialEq, Clone)]
enum State {
    Zero,
    One,
    Two,
    Rejected,
}
pub fn reject_three(input: &str) -> bool{
    use State::*;
    let mut state = Zero;

    for ch in input.chars() {
        state = match (state.clone(), ch) {
            (Zero, '1') => One,
            (Zero, '0') => Zero,
            (One, '0') => Zero,
            (One, '1') => Two,
            (Two, '0') => Zero,
            (Two, '1') => Rejected,
            (Rejected, _) => Rejected,  // Stay in Rejected state regardless of input
            _ => state,  // Catch-all
        };

        match state {
            Zero => println!("Zero"),
            One => println!("One"),
            Two => println!("Two"),
            Rejected => println!("Rejected")
        }
    }
    state != Rejected
}