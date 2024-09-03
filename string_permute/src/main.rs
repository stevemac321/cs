fn permute(chars: &mut Vec<char>, left: usize, right: usize) {
    if left == right {
        println!("{}", chars.iter().collect::<String>());
    } else {
        for i in left..=right {
            chars.swap(left, i);
            permute(chars, left + 1, right);
            chars.swap(left, i);  // backtrack
        }
    }
}

fn main() {
    let mut chars: Vec<char> = "abc".chars().collect();
    let n = chars.len();
    permute(&mut chars, 0, n - 1);
}

