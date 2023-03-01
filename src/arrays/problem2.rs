/*
        ************* Remove Element *************

Given an integer array "nums" and an integer "val", remove all occurrences of "val" in "nums" in-place. 
The relative order of the elements may be changed.

Since it is impossible to change the length of the array in some languages, you must instead have the 
result be placed in the first part of the array "nums". More formally, if there are "k" elements after removing 
the duplicates, then the first "k" elements of "nums" should hold the final result. It does not matter what you 
leave beyond the first "k" elements.

Return "k" after placing the final result in the first "k" slots of "nums".

Do not allocate extra space for another array. You must do this by modifying the input array in-place with 
O(1) extra memory.

Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int val = ...; // Value to remove
int[] expectedNums = [...]; // The expected answer with correct length.
                            // It is sorted with no values equaling val.

int k = removeElement(nums, val); // Calls your implementation

assert k == expectedNums.length;
sort(nums, 0, k); // Sort the first k elements of nums
for (int i = 0; i < actualLength; i++) {
    assert nums[i] == expectedNums[i];
}
If all assertions pass, then your solution will be accepted.

 

Example 1:

Input: nums = [3,2,2,3], val = 3
Output: 2, nums = [2,2,_,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 2.
It does not matter what you leave beyond the returned k (hence they are underscores).
Example 2:

Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5, nums = [0,1,4,0,3,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
Note that the five elements can be returned in any order.
It does not matter what you leave beyond the returned k (hence they are underscores).
 

Constraints:

0 <= nums.length <= 100
0 <= nums[i] <= 50
0 <= val <= 100
*/


pub fn remove_elem(nums: &mut Vec<i32>, val: i32) -> i32 {
    while let Some(index) = nums.iter().position(|v| *v == val) {
        nums.swap_remove(index);
    }
    nums.len() as i32
}

pub fn remove_elem_2(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rm_elem_works() {
        let mut arr_nums = vec![3,2,2,3];
        let actual_result = remove_elem(&mut arr_nums, 3);
        let accepted_result = 2;
        assert_eq!(actual_result, accepted_result);
    }

    #[test]
    fn remove_element_works2() {
        let mut arr_nums = vec![0,1,2,2,3,0,4,2];
        let actual_result = remove_elem_2(&mut arr_nums, 2);
        let accepted_result = 5;
        assert_eq!(actual_result, accepted_result);
    }

}