impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complement_map = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let compl = target - num;
            if complement_map.contains_key(&compl) {
                return vec![
                    *complement_map.get(&compl).unwrap(),
                    idx.try_into().unwrap()
                ];
            }
            complement_map.insert(num, idx.try_into().unwrap());
        }

        vec![]
    }
}
