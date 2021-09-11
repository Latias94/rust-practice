struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut total_len: i32 = 0;
        let map = Self::create_map_from_string(&chars);
        'outer: for word in words {
            let word_map = Self::create_map_from_string(&word);
            for (c, count) in word_map {
                if map.get(&c).unwrap_or(&0) < &count {
                    continue 'outer;
                }
            }
            total_len += word.len() as i32;
        }
        total_len
    }

    pub fn create_map_from_string(word: &str) -> HashMap<u8, u32> {
        word.bytes()
            .into_iter()
            .fold(HashMap::new(), |mut map, num| {
                *map.entry(num).or_insert(0) += 1;
                map
            })
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use rustgym_util::*;

    use super::*;

    #[test]
    fn test() {
        let words = vec_string!["cat", "bt", "hat", "tree"];
        let chars = "atach".to_string();
        assert_eq!(Solution::count_characters(words, chars), 6);
        let words = vec_string!["hello", "world", "leetcode"];
        let chars = "welldonehoneyr".to_string();
        assert_eq!(Solution::count_characters(words, chars), 10);
    }
}
