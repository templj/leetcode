impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);

        for row_num in 0..num_rows as usize {
            let mut row = Vec::with_capacity(row_num + 1);

            // First element of each row is always 1
            row.push(1);

            // Add interior elements from the previous row
            if row_num > 0 {
                for j in 1..row_num {
                    let prev_row = &result[row_num - 1];
                    row.push(prev_row[j - 1] + prev_row[j]);
                }
            }

            // Last element is always 1 (except when the row has only one element)
            if row_num > 0 {
                row.push(1);
            }

            result.push(row);
        }

        result
    }
}
