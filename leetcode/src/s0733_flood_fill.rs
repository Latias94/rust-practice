struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let usr = sr as usize;
        let usc = sc as usize;
        let old_color = image[usr][usc];
        image[usr][usc] = new_color;
        if old_color != new_color {
            if sr > 0 && image[usr - 1][usc] == old_color {
                image = Self::flood_fill(image, sr - 1, sc, new_color);
            }
            if sc > 0 && image[usr][usc - 1] == old_color {
                image = Self::flood_fill(image, sr, sc - 1, new_color);
            }
            if usr < image.len() - 1 && image[usr + 1][usc] == old_color {
                image = Self::flood_fill(image, sr + 1, sc, new_color);
            }
            if usc < image[0].len() - 1 && image[usr][usc + 1] == old_color {
                image = Self::flood_fill(image, sr, sc + 1, new_color);
            }
        }

        image
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let new_color = 2;
        let new_image = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(Solution::flood_fill(image, sr, sc, new_color), new_image);

        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let sr = 0;
        let sc = 0;
        let new_color = 2;
        let new_image = vec![vec![2, 2, 2], vec![2, 2, 2]];
        assert_eq!(Solution::flood_fill(image, sr, sc, new_color), new_image);
    }
}
