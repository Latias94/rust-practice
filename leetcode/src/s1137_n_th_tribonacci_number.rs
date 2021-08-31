struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut v = vec![0, 1, 1];
        loop {
            let i = n as usize;
            match v.get(i) {
                Some(j) => {
                    return *j;
                }
                None => {
                    let sum = v[v.len() - 1] + v[v.len() - 2] + v[v.len() - 3];
                    v.push(sum);
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
