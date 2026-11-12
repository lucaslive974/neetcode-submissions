use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_map = HashMap::new();
        for c in s.chars() { *s_map.entry(c).or_insert(0) += 1; }

        let mut t_map = HashMap::new();
        for c in t.chars() { *t_map.entry(c).or_insert(0) += 1; }

        s_map == t_map
    }
}