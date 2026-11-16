impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let build_map = |mut map: HashMap<char, u32>, word: String| {
            for character in word.chars() {
                *map.entry(character).or_insert(0) += 1;
            }
            return map;
        };

        let s_map = build_map(HashMap::new(), s);
        let mut t_map = build_map(HashMap::new(), t);

        for (key, count) in s_map {
            let t_entry = t_map.entry(key);
            if *t_entry.or_insert(0) != count {
                return false;
            }
        }

        true
    }
}
