/*
        ************* Sort an Array *************

Given an array of integers nums, sort the array in ascending order and return it.

You must solve the problem without using any built-in functions in O(nlog(n)) time complexity 
and with the smallest space complexity possible.


Example 1:

Input: nums = [5,2,3,1]
Output: [1,2,3,5]
Explanation: After sorting the array, the positions of some numbers are not changed 
(for example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).
Example 2:

Input: nums = [5,1,1,2,0,0]
Output: [0,0,1,1,2,5]
Explanation: Note that the values of nums are not necessairly unique.
 

Constraints:

1 <= nums.length <= 5 * 104
-5 * 104 <= nums[i] <= 5 * 104 
*/

pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }

    let mid = nums.len() / 2;
    let (left, right) = nums.split_at(mid);

    let left_sorted = sort_array(left.to_vec());
    let right_sorted = sort_array(right.to_vec());

    let mut sorted = Vec::with_capacity(nums.len());
    let mut i = 0;
    let mut j = 0;

    while i < left_sorted.len() && j < right_sorted.len() {
        if left_sorted[i] <= right_sorted[j] {
            sorted.push(left_sorted[i]);
            i += 1;
        } else {
            sorted.push(right_sorted[j]);
            j += 1;
        }
    }

    sorted.extend_from_slice(&left_sorted[i..]);
    sorted.extend_from_slice(&right_sorted[j..]);

    sorted
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array() {
        let nums = vec![5, 2, 3, 1];
        assert_eq!(sort_array(nums), vec![1, 2, 3, 5]);

        let nums = vec![5, 1, 1, 2, 0, 0];
        assert_eq!(sort_array(nums), vec![0, 0, 1, 1, 2, 5]);
    }
}
