struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        if let Some(pos) = chars.iter().position(|&c| c == ch) {
            chars[..=pos].reverse();
        }
        chars.into_iter().collect()
    }
}
