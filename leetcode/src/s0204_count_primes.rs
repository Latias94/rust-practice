struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        // 埃拉托斯特尼筛法
        if n <= 2 {
            return 0;
        }
        let mut nums = vec![true; n as usize];
        nums[0] = false;
        nums[1] = false;
        let sq = f32::sqrt(n as f32) as usize;
        let mut i = 2;
        while i <= sq {
            if nums[i] {
                let mut j = 2;
                while j * i < n as usize {
                    nums[j * i] = false;
                    j += 1;
                }
            }

            i += 1;
        }
        nums.into_iter().filter(|x| *x).count() as i32
    }

    // Time Limit Exceeded
    // pub fn count_primes(n: i32) -> i32 {
    //     let mut count = 0;
    //     for i in 2..n {
    //         let sq = f32::sqrt(i as f32) as i32;
    //         let mut t = 2;
    //         let mut is_prime = true;
    //         while t <= sq {
    //             if i % t == 0 {
    //                 is_prime = false;
    //                 break;
    //             }
    //             t += 1;
    //         }
    //         if is_prime {
    //             count += 1;
    //         }
    //     }
    //     count
    // }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;
    // use rustgym_util::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
        assert_eq!(Solution::count_primes(2), 0);
        assert_eq!(Solution::count_primes(3), 1);
        assert_eq!(Solution::count_primes(10), 4);
    }
}
