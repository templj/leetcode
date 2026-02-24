struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut heights_sorted = heights.clone();
        heights_sorted.sort_unstable();
        heights.iter()
            .zip(&heights_sorted)
            .filter(|(a, b)| a != b)
            .count() as i32
    }
}


fn main() {
    let heights = vec![1,1,4,2,1,3];
    let result = Solution::height_checker(heights);
    println!("{} expected 3", result);
}
