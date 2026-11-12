impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();

        for num in &nums {
            if set.contains(num){ 
                return true; 
            }

            set.insert(num);
        }

        return false;
    }
}
