/*
SEC. 10.3 DETERMINISTIC AND NONDETERMINISTIC AUTOMATA 537
One of the most basic operations using an automaton is to take a sequence of
symbols a1a2 · · · ak and follow from the start state a path whose arcs have labels
that include these symbols in order. That is, for i = 1, 2, . . . , k, ai is a member of the
set Si that labels the ith arc of the path. Constructing this path and its sequence
of states is called simulating the automaton on the input sequence a1a2 · · · ak. This
path is said to have the label a1 a2 · · · ak; it may also have other Label of a path labels, of course,
since the sets Si labeling the arcs along the path may each include many characters.
F Example 10.3. We did one such simulation in Fig. 10.5, where we followed the
automaton of Fig. 10.4 on input sequence 0101101. For another example, consider
the automaton of Fig. 10.3, which we used to recognize words with subsequence
aeiou. Consider the character string adept.
We start in state 0. There are two transitions out of state 0, one on the set of
characters Alpha(A) − a (means A minus "a", all chars except "a"), and the other on a alone. Since the first character 
in adept is a,we follow the latter transition, which takes us to state 1. Out of state 1, there are
transitions on Alpha(A)−e and e. Since the second character is d, we must take the former
transition, because all letters but e are included in the set Alpha(A) − e. That leaves us
in state 1 again. As the third character is e, we follow the second transition out of
state 1, which takes us to state 2. The final two letters of adept are both included
in the set Alpha(A) − i, and so our next two transitions are from state 2 to state 2. We
thus finish our processing of adept in state 2. The sequence of state transitions is
shown in Fig. 10.6. Since state 2 is not an accepting state, we do not accept the
input adept. F
Input:  a  d  e  p  t
State: 0 1  1  2  2  2
Fig. 10.6. Simulation of the automaton in Fig. 10.3 on input adept
*/

#[derive(Debug, PartialEq, Clone)]
enum State {
    Start,
    A,
    E,
    I,
    O,
    U,
    Rejected,
}

pub fn inorder(words: &[char]) -> bool {
    use State::*;

    let mut state = Start;

    for &c in words {
        state = match (state.clone(), c.to_ascii_lowercase()) {
            (Start, 'a') => A,
            (A, 'e') => E,
            (E, 'i') => I,
            (I, 'o') => O,
            (O, 'u') => U,
            (_, _) if !['a', 'e', 'i', 'o', 'u'].contains(&c.to_ascii_lowercase()) => state,
            _ => Rejected,
        };
        if state == Rejected {
            return false;  // Immediate rejection on out-of-order input
        }
    }

    state == U  // Only accept if the final state is `U`
}
