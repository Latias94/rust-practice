struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|a| a[1]);

        let mut result = 1;
        let mut prev = &points[0][1];
        for point in &points {
            if prev < &point[0] {
                result += 1;
                prev = &point[1];
            }
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let points1 = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(Solution::find_min_arrow_shots(points1), 2);

        let points2 = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(Solution::find_min_arrow_shots(points2), 4);

        let points3 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::find_min_arrow_shots(points3), 2);

        let points4 = vec![vec![1, 2]];
        assert_eq!(Solution::find_min_arrow_shots(points4), 1);
    }
}