
enum State {
   A,
   B,
   C,
   D,
}

pub fn bounce(input: &str) {
   let mut state = State::A;

   for ch in input.chars() {
       match state {
           State::A => {
               print!("0");
               if ch == '0' {
                   state = State::A;
               } else if ch == '1' {
                   state = State::B;
               }
           }
           State::B => {
               print!("0");
               if ch == '0' {
                   state = State::A;
               } else if ch == '1' {
                   state = State::C;
               }
           }
           State::C => {
               print!("1");
               if ch == '0' {
                   state = State::D;
               } else if ch == '1' {
                   state = State::C;
               }
           }
           State::D => {
               print!("1");
               if ch == '0' {
                   state = State::A;
               } else if ch == '1' {
                   state = State::C;
               }
           }
       }
   }
}

/*
enum State {
    A,
    B,
    C,
    D,
}

impl State {
    fn state_a(ch: char) -> State {
        print!("0");
        if ch == '0' {
            State::A
        } else {
            State::B
        }
    }

    fn state_b(ch: char) -> State {
        print!("0");
        if ch == '0' {
            State::A
        } else {
            State::C
        }
    }

    fn state_c(ch: char) -> State {
        print!("1");
        if ch == '0' {
            State::D
        } else {
            State::C
        }
    }

    fn state_d(ch: char) -> State {
        print!("1");
        if ch == '0' {
            State::A
        } else {
            State::C
        }
    }
}

pub fn bounce(input: &str) {
    let mut state = State::A;

    for ch in input.chars() {
        state = match state {
            State::A => State::state_a(ch),
            State::B => State::state_b(ch),
            State::C => State::state_c(ch),
            State::D => State::state_d(ch),
        };
    }
}
    */
