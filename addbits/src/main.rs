fn addbits(a: u32, b: u32) -> u32 {
    let mut c: u32 = 0;
    let mut carry_flag: bool = false;
    let mut sum = 0;

    for i in 0..32 {
        let bit_a = getbit(&a, i);
        let bit_b = getbit(&b, i);
       
        match (carry_flag, bit_a, bit_b) {
            (true, 0, 0) => {
                sum = 1;
                carry_flag = false;
            },
            (true, 1, 0) | (true, 0, 1) => {
                sum = 1;
                carry_flag = true;
            },
            (true, 1, 1) => {
                sum = 0;
                carry_flag = true;
            },
            (false, 0, 0) => {
                sum = 0;
                carry_flag = false;
            },
            (false, 1, 0) | (false, 0, 1) => {
                sum = 1;
                carry_flag = false;
            },
            (false, 1, 1) => {
                sum = 0;
                carry_flag = true;
            },
            _ => unreachable!(), // This should never be hit if x and y are always binary
        }

        setbit(&mut c, i, sum);
    }
    if carry_flag {
        setbit(&mut c, 31, 1);
    }
    c
}

fn main() {
    let a: u32 = 7;
    let b: u32 = 15;
    let c = addbits(a, b);
    println!("{:032b}", c);
}

fn getbit(num: &u32, pos: u32) -> u32 {
    (num >> pos) & 1
}

fn setbit(num: &mut u32, pos: u32, value: u32) {
    let mask = 1 << pos;

    if value == 1 {
        *num |= mask;  // Correctly modify the value at the reference
    } else {
        *num &= !mask; // Correctly modify the value at the reference
    }
}







