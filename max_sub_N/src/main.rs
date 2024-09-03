//  O(n) solution, Kadane's algorithm, maximun contiguous subarray, buy low, sell high.
// I got it from Mark Allen Weiss book
// the answer is (43, 7, 10)
fn max_sub(arr: &[i32]) ->(i32, usize, usize) {
    let mut max_sum = 0;
    let mut this_sum = 0;
    let mut i = 0; 
    let mut start = 0;
    let mut end = 0;

    for mut j in 0..arr.len() {
        this_sum += arr[j];

        if this_sum > max_sum {
            max_sum = this_sum;
            start = i;
            end = j;
        } else if this_sum < 0 {
            i = j + 1;
            this_sum = 0;
        }
    }
    (max_sum, start, end)
}
fn main() {
    let arr: &[i32; 16] =  &[13,-3,-25,20,-3,-16,-23,18,20,-7,12,-5,-22,15,-4,7];
    let (x, y, z) = max_sub(arr);
    println!("{:?}",(x,y,z));
}

/*
Kadane's algorithm, maximum subarray problem with a time complexity of \(O(n)\). Here's a breakdown of how the code works:

### Explanation:
1. **Initialization**:
   - `max_sum`: Tracks the maximum sum found so far.
   - `this_sum`: Tracks the sum of the current subarray.
   - `i`: Tracks the starting index of the current subarray.
   - `start` and `end`: Track the start and end indices of the maximum subarray found.

2. **Iterating Through the Array**:
   - The loop iterates over each element of the array, adding the current element to `this_sum`.
   - If `this_sum` is greater than `max_sum`, it updates `max_sum` and the `start` and `end` indices to reflect the current subarray.
   - If `this_sum` drops below 0, it means the current subarray is not contributing to a maximum sum, so `this_sum` is reset to 0, and the starting index `i` is updated to the next element.

3. **Return Value**:
   - The function returns a tuple containing the maximum sum and the start and end indices of the subarray that produces this sum.

### Output:
For the provided array `[13,-3,-25,20,-3,-16,-23,18,20,-7,12,-5,-22,15,-4,7]`, the code correctly identifies the maximum subarray as `[18,20,-7,12]` with a sum of `43`, starting at index `7` and ending at index `10`.

*
*
* */
