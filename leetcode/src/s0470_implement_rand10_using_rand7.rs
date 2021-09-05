struct Solution;

use rand::distributions::Uniform;
use rand::thread_rng;
use rand::Rng;

fn rand7() -> i32 {
    let distribution: Uniform<i32> = Uniform::new(0, 7);
    let mut rng = thread_rng();
    rng.sample(distribution) + 1
}

//leetcode submit region begin(Prohibit modification and deletion)
/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        loop {
            let result = (rand7() - 1) * 7 + rand7();
            if result <= 40 {
                return result % 10 + 1;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
