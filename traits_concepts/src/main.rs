use std::ops::{BitAnd, BitOr, BitXor, Shr, Shl, Add, Not, BitOrAssign};

fn add_bits<T>(a: T, b: T) -> (T, T)
where
    T: Default
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Shr<usize, Output = T>
        + Shl<usize, Output = T>
        + Copy
        + Not<Output = T>
        + From<u8>
        + Add<Output = T>
        + BitOrAssign,
{
    let mut sum = T::default();
    let mut carry = T::default();
    let mut c_in = T::default();
    let bit_count = std::mem::size_of::<T>() * 8;

    for i in 0..bit_count {
        let a_bit = (a >> i) & T::from(1);
        let b_bit = (b >> i) & T::from(1);

        let sum_bit = a_bit ^ b_bit ^ c_in;
        carry = (a_bit & b_bit) | (b_bit & c_in) | (c_in & a_bit);

        sum |= sum_bit << i;
        c_in = carry;
    }

    (sum, c_in)
}

fn main() {
    let a: u32 = 0b1010; // Example: 10 in binary
    let b: u32 = 0b1100; // Example: 12 in binary

    let (sum, carry) = add_bits(a, b);

    println!("Sum: {:#b}, Carry: {:#b}", sum, carry);
}
