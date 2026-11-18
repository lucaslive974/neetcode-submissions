impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashSet;
        let mut stack = Vec::with_capacity(s.len());
        let op_map = HashSet::from(['{', '[', '(']);
        let pair_map = HashMap::from([('}', '{'), (']', '['), (')', '(')]);

        for c in s.chars() {
            if op_map.contains(&c) {
                stack.push(c);
                continue;
            }
            
            if stack.is_empty() {
                return false;
            }

            if let Some(last) = stack.pop() {
                if pair_map.get(&c).unwrap() != &last {
                    return false;
                }
            }
        }
        
        if !stack.is_empty() {
            return false;
        }

        true
    }
}
