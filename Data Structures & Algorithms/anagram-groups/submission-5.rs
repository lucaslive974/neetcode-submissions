impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for str in &strs {
            let mut chars = str.chars().collect::<Vec<char>>();
            chars.sort_unstable();

            let sstr = chars.into_iter().collect::<String>();

            groups.entry(sstr).or_default().push(str.clone());
        }

        groups.into_values().collect()
    }
}
