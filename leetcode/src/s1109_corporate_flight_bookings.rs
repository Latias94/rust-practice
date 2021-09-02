struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut answer = vec![0; n as usize];
        for booking in bookings {
            answer[booking[0] as usize - 1] += booking[2];
            if booking[1] < n {
                answer[booking[1] as usize] -= booking[2];
            }
        }
        for i in 1..n as usize {
            answer[i] += answer[i - 1];
        }
        answer
    }

    #[allow(dead_code)]
    pub fn corp_flight_bookings_bruce(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut answer = vec![0; n as usize];
        for booking in bookings {
            for flight in booking[0]..=booking[1] {
                answer[flight as usize - 1] += booking[2];
            }
        }
        answer
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
        let n = 5;
        let res = vec![10, 55, 45, 25, 25];
        assert_eq!(Solution::corp_flight_bookings(bookings, n), res);
    }
}
