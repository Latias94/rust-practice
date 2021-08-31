struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_content_children(mut children: Vec<i32>, mut cookies: Vec<i32>) -> i32 {
        children.sort_unstable();
        cookies.sort_unstable();
        let mut child = 0;
        let mut cookie = 0;
        while child < children.len() && cookie < cookies.len() {
            if children[child] <= cookies[cookie] {
                child += 1;
            }
            cookie += 1;
        }
        child as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        let res = 1;
        assert_eq!(Solution::find_content_children(g, s), res);
        let g = vec![1, 2];
        let s = vec![1, 2, 3];
        let res = 2;
        assert_eq!(Solution::find_content_children(g, s), res);
    }
}
