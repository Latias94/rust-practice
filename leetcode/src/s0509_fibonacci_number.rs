struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn fib(n: i32) -> i32 {
        Self::fib_dynamic(n).0
    }

    pub fn fib_dynamic(n: i32) -> (i32, i32) {
        if n == 0 {
            return (0, 0);
        }
        if n == 1 {
            return (1, 0);
        }
        let (a, b) = Self::fib_dynamic(n - 1);
        (a + b, a)
    }

    #[allow(dead_code)]
    pub fn fib_store_with_vec(n: i32) -> i32 {
        let mut v = vec![0, 1];
        loop {
            let i = n as usize;
            match v.get(i) {
                Some(j) => {
                    return *j;
                }
                None => {
                    let sum = v[v.len() - 1] + v[v.len() - 2];
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
    fn fibonacci_number_1() {
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
        assert_eq!(Solution::fib(10), 55);
        assert_eq!(Solution::fib(20), 6765);
    }
}
