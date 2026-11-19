impl Solution {
    fn solve_search(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
        if start >= end {
            return -1;
        }

        let middle = (end - start) / 2 + start;

        let value = nums[middle];
        match value {
            x if x == target => return middle as i32,
            x if x > target => {
                return self::Solution::solve_search(nums, target, start, middle);
            }
            _ => return self::Solution::solve_search(nums, target, middle + 1, end),
        };
    } 

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        self::Solution::solve_search(&nums, target, 0, nums.len())
    }
}
