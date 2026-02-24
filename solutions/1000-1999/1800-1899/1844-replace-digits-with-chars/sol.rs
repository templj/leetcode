struct Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if chars[i].is_digit(10) {
                let prev_char = chars[i - 1];
                let digit_value = chars[i].to_digit(10).unwrap();
                let new_char = ((prev_char as u8) + (digit_value as u8)) as char;
                result.push(new_char);
            } else {
                result.push(chars[i]);
            }
        }
        result
    }
}

fn main() {
    let s = "a1c1e1".to_string();
    let result = Solution::replace_digits(s);
    println!("{} expected abcdef", result);
}
