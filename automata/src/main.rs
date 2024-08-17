extern crate automata;
use automata::bounce::bounce;
use automata::inorder::inorder;
use automata::parity::parity;
use automata::reject_three::reject_three;
use regex::Regex;

fn main() {
    test_bounce();
	test_inorder();
    println!("\n");
    test_parity();
    test_reject_three();
	test_bounce_regex();

}

fn test_inorder() {
    let word1: [char; 10] = ['a', 'b', 's', 't', 'e', 'm', 'i', 'o', 'u', 's'];
    let word2: [char; 9]  = ['f', 'a', 'c', 'e', 't', 'i', 'o', 'u', 's'];
    let word3: [char; 8]  = ['a', 'e', 'r', 'i', 'f', 'o', 'r', 'm'];
    let word4: [char; 9]  = ['a', 'r', 's', 'e', 'n', 'i', 'o', 'u', 's'];
    let word5: [char; 8]  = ['a', 'n', 'e', 'm', 'i', 'o', 'u', 's'];
    let word6: [char; 7]  = ['c', 'a', 'e', 's', 'i', 'u', 'm'];
    let word7: [char; 5]  = ['b', 'a', 'c', 'o', 'n'];
    let word8: [char; 5]  = ['q', 'u', 'e', 'u', 'e'];
    let word9: [char; 8]  = ['a', 'e', 'i', 'a', 'e', 'i', 'o', 'u' ];
    

    let mut ret = inorder(&word1);
    assert_eq!(ret, true);
    
    ret = inorder(&word2);
    assert_eq!(ret, true);

    ret = inorder(&word3);
    assert_eq!(ret, false);

    ret = inorder(&word4);
    assert_eq!(ret, true);

    ret = inorder(&word5);
    assert_eq!(ret, true);

    ret = inorder(&word6);
    assert_eq!(ret, false);

    ret = inorder(&word7);
    assert_eq!(ret, false);

    ret = inorder(&word8);
    assert_eq!(ret, false);

    ret = inorder(&word9);
    assert_eq!(ret, false);
    
}
fn test_bounce() {
    let input = "0101101";
    bounce(input);
}
fn test_parity() {
    let input = "0110";
    let mut ret = parity(input);
    assert_eq!(ret, true);

    let input2 = "010";
    ret = parity(input2);
    assert_eq!(ret, false);
}
fn test_reject_three() {
    let mut input = "101001101110";
    assert_eq!(reject_three(input), false); // Regular case with three consecutive 1's
    
    input = "10100110110";
    assert_eq!(reject_three(input), true); // Regular case without three consecutive 1's
    
    // Edge Case 1: Empty String
    input = "";
    assert_eq!(reject_three(input), true); // No input should be accepted (no three consecutive 1's)
    
    // Edge Case 2: Single Character '0'
    input = "0";
    assert_eq!(reject_three(input), true); // Only a single 0 should be accepted
    
    // Edge Case 3: Single Character '1'
    input = "1";
    assert_eq!(reject_three(input), true); // Only a single 1 should be accepted (not three consecutive 1's)
    
    // Edge Case 4: Two Characters '00'
    input = "00";
    assert_eq!(reject_three(input), true); // Two 0's should be accepted
    
    // Edge Case 5: Two Characters '11'
    input = "11";
    assert_eq!(reject_three(input), true); // Two 1's should be accepted
    
    // Edge Case 6: Three Consecutive '1's at Start
    input = "111";
    assert_eq!(reject_three(input), false); // Three consecutive 1's should be rejected
    
    // Edge Case 7: Three Consecutive '1's at End
    input = "00111";
    assert_eq!(reject_three(input), false); // Three consecutive 1's should be rejected
    
    // Edge Case 8: Large Input with No Consecutive '1's
    input = "0101010101010101010101010101010101010";
    assert_eq!(reject_three(input), true); // Should be accepted (no three consecutive 1's)
    
    // Edge Case 9: Large Input with Three Consecutive '1's in the Middle
    input = "010101011110101010101010101010101010";
    assert_eq!(reject_three(input), false); // Should be rejected (contains three consecutive 1's)
    
}
/*
Full regular expression is:
(0|1)*11(1|01)*(e|0)  see page 562 in Foundations.  In short, this regex represents any string of
1's and 0's followed by two 1's.
*/
fn test_bounce_regex() {
 let re = Regex::new(r"^(0|1)*11$").unwrap();
    
    // The string to test
    let test_string = "01011";
    
    // Check if the string matches the regex
    if re.is_match(test_string) {
        println!("The string '{}' matches the pattern", test_string);
    } else {
        println!("The string '{}' does not match the pattern", test_string);
    }
}
