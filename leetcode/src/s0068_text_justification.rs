struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let n = words.len();
        let max_width: usize = max_width as usize;
        let mut result = Vec::new();
        let mut left = 0;
        let mut right = 1;
        while left < right && right <= n {
            let mut sub_len = words[left].len();
            while right < n && sub_len + 1 + words[right].len() <= max_width {
                sub_len += 1 + words[right].len();
                right += 1;
            }

            // 除了两个单词之间的一个空格外，这一行还要放多少空格
            let mut space_left = if max_width > sub_len {
                max_width - sub_len
            } else {
                0
            };
            let mut row = "".to_string();
            if left < right && right < n {
                let len = right - left;
                let one_element_only = right - left - 1 == 0; // 是不是只放得下一个元素
                                                              // 除了两个单词之间的一个空格外，平均每个间隔里面要多少空格
                let each_space = if one_element_only {
                    0
                } else {
                    space_left / (len - 1)
                };
                // 除了所有单词之间的一个空格，和整除放的空格外，还剩下多少空格
                let mut space_remain = if one_element_only {
                    0
                } else {
                    space_left % (len - 1)
                };
                if words[left].len() <= max_width {
                    row.push_str(&words[left].clone());
                }
                for i in (left + 1)..right {
                    for _ in 0..each_space {
                        row.push(' ');
                    }
                    if space_left >= each_space {
                        space_left -= each_space;
                    }
                    if space_remain > 0 {
                        row.push(' ');
                        space_remain -= 1;
                    }
                    row.push(' '); // 两个单词之间固定放的空格
                    row.push_str(&words[i].clone());
                }
                if one_element_only {
                    // 如果这行只有一个单词，就在尾部填满空格
                    for _ in row.len()..max_width {
                        row.push(' ');
                    }
                }
            } else {
                // 最后一行
                row.push_str(&words[left].clone());
                for i in (left + 1)..right {
                    row.push(' ');
                    row.push_str(&words[i].clone());
                }

                for _ in row.len()..max_width {
                    row.push(' ');
                }
            }
            left = right;
            right = left + 1;
            result.push(row);
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;
    use rustgym_util::*;

    #[test]
    fn test() {
        let words = vec_string!["Listen", "to", "many,", "speak", "to", "a", "few."];
        let max_width = 6;
        let res = vec_string!["Listen", "to    ", "many, ", "speak ", "to   a", "few.  "];
        assert_eq!(Solution::full_justify(words, max_width), res);
        let words = vec_string!["a"];
        let max_width = 2;
        let res = vec_string!["a "];
        assert_eq!(Solution::full_justify(words, max_width), res);
        let words = vec_string![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification."
        ];
        let max_width = 16;
        let res = vec_string!["This    is    an", "example  of text", "justification.  "];
        assert_eq!(Solution::full_justify(words, max_width), res);
        let words = vec_string!["What", "must", "be", "acknowledgment", "shall", "be"];
        let max_width = 16;
        let res = vec_string!["What   must   be", "acknowledgment  ", "shall be        "];
        assert_eq!(Solution::full_justify(words, max_width), res);
        let words = vec_string!["What", "must", "be", "acknowledgment", "shall", "be"];
        let max_width = 16;
        let res = vec_string!["What   must   be", "acknowledgment  ", "shall be        "];
        assert_eq!(Solution::full_justify(words, max_width), res);
        let words = vec_string![
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do"
        ];
        let max_width = 20;
        let res = vec_string![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ];
        assert_eq!(Solution::full_justify(words, max_width), res);
    }
}
