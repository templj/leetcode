impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let xdist: i32 = (x - z).abs();
        let ydist: i32 = (y - z).abs();
        if xdist < ydist {
            return 1
        } else if ydist < xdist {
            return 2
        } else {
            return 0
        }
    }
}
