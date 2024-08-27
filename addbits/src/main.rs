fn addbits(a: u32, b: u32) -> u32 {
    let mut c: u32 = 0;
    let mut carry_flag = 0;
    let mut sum = 0;

    for i in 0..32 {
        let bit_a = getbit(&a, i);
        let bit_b = getbit(&b, i);
        let state = bit_a + bit_b + carry_flag;
       
        match (state) {
            (0) => {
                sum = 0;
                carry_flag = 0;
            },
            (1) => {
                sum = 1;
                carry_flag = 0;
            },
            (2) => {
                sum = 0;
                carry_flag = 1;
            },
            (3) => {
                sum = 1;
                carry_flag = 1;
            },
            _ => unreachable!(), // This should never be hit if x and y are always binary
        }

        setbit(&mut c, i, sum);
    }
    if carry_flag == 1 {
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







