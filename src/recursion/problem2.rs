/* 
        ************* Climbing Stairs *************

You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

 

Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps
Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step
 

Constraints:

1 <= n <= 45
*/

pub fn climb_stairs(n: i32) -> i32 {
    if n <= 3 {
    return n;
}
    
    let mut first = 1;
    let mut second = 2;
    
    let mut result = 0;
    
    for _ in 2..n {
        result = first + second;
        first = second;
        second = result;
    }
    
    result
}


pub fn climb_stairs_complex(n: i32) -> i32 {
    std::iter::successors(Some((0, 1)), |dp| Some((dp.1, dp.0 + dp.1)))
        .take((n + 1) as usize)
        .last()
        .unwrap()
        .1
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_limbs() {
        let actual_climbs = climb_stairs(7);
        let expected_climbs = 21;
        assert_eq!(actual_climbs, expected_climbs);
    }

    #[test]
    fn check_limbs_complex() {
        let actual_climbs = climb_stairs_complex(7);
        let expected_climbs = 21;
        assert_eq!(actual_climbs, expected_climbs);
    }
}