impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();

        for num in nums {
            count
                .entry(num)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }

        let mut vec = count.into_iter().collect::<Vec<(i32, i32)>>();
        vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut res = Vec::new();
        for el in vec.into_iter().take(k.try_into().unwrap()) {
            res.push(el.0);
        }

        res
    }
}