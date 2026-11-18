impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '{' | '[' | '(' => {
                    stack.push(c);
                    continue;
                }
                _ => (),
            }

            match stack.pop() {
                Some(last) => match (last, c) {
                    ('(', ')') | ('[', ']') | ('{', '}') => (),
                    _ => return false,
                },
                None => return false,
            }
        }

        if !stack.is_empty() {
            return false;
        }

        true
    }
}
