struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut counter: i32 = 0;
        let mut result: String = String::new();
        for c in s.chars() {
            if c == '(' {
                if counter > 0 {
                    result.push(c);
                }
                counter += 1;
            } else {
                counter -= 1;
                if counter > 0 {
                    result.push(c);
                }
            }
        }
        result
    }
}
