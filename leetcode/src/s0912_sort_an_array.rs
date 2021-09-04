struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::quick_sort(&mut nums[..]);
        nums
    }

    // 归并排序
    #[allow(dead_code)]
    pub fn merge_sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
        if v.len() <= 1 {
            return v;
        }

        let mut result = Vec::with_capacity(v.len());
        let b = v.split_off(v.len() >> 1);
        let a = Self::merge_sort(v);
        let b = Self::merge_sort(b);

        // bring together
        let mut a_it = a.into_iter();
        let mut b_it = b.into_iter();
        let mut a_peek = a_it.next();
        let mut b_peek = b_it.next();
        loop {
            match a_peek {
                Some(ref a_val) => match b_peek {
                    Some(ref b_val) => {
                        if b_val < a_val {
                            result.push(b_peek.take().unwrap());
                            b_peek = b_it.next();
                        } else {
                            result.push(a_peek.take().unwrap());
                            a_peek = a_it.next();
                        }
                    }
                    None => {
                        result.push(a_peek.take().unwrap());
                        result.extend(a_it);
                        return result;
                    }
                },
                None => {
                    if let Some(b_val) = b_peek {
                        result.push(b_val);
                    }
                    result.extend(b_it);
                    return result;
                }
            }
        }
    }

    // 快速排序
    pub fn quick_sort<T: PartialOrd>(v: &mut [T]) {
        if v.len() <= 1 {
            return;
        }
        let p = Self::pivot(v);
        let (a, b) = v.split_at_mut(p);
        Self::quick_sort(a);
        Self::quick_sort(&mut b[1..]); // Middle element already sorted
    }

    // using rayon for multi-threading
    // pub fn quick_sort_rayon<T: Send + PartialOrd>(v: &mut [T]) {
    //     if v.len() <= 1 {
    //         return;
    //     }
    //     let p = pivot(v);
    //     let (a, b) = v.split_at_mut(p);
    //     rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
    // }

    // 线程数有可能用太多 可以用 Rayon
    #[allow(dead_code)]
    pub fn threaded_quick_sort<T: 'static + PartialOrd + Send>(v: &mut [T]) {
        if v.len() <= 1 {
            return;
        }
        let p = Self::pivot(v);
        let (a, b) = v.split_at_mut(p);

        let raw_a: *mut [T] = a as *mut [T];
        let raw_s = RawSend(raw_a);

        unsafe {
            let handle = std::thread::spawn(move || {
                Self::threaded_quick_sort(&mut *raw_s.0);
            });
            Self::threaded_quick_sort(&mut b[1..]); // Middle element already sorted
            handle.join().ok();
        }
    }

    // Move first element to the correct place
    // Everything lower should be before it.
    // Everything higher should be after it.
    // return it's location
    pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
        let mut r = RandGen::new(v.len()); // not truly random
        let mut p = r.next_v(v.len());
        v.swap(p, 0);
        p = 0;
        for i in 1..v.len() {
            if v[i] < v[p] {
                // move our pivot forward 1, and put this element before it
                v.swap(p + 1, i);
                v.swap(p, p + 1);
                p += 1;
            }
        }
        p
    }

    // 冒泡排序 Time Limit Exceeded
    #[allow(dead_code)]
    pub fn bubble_sort<T: PartialOrd>(v: &mut Vec<T>) {
        for p in 0..v.len() {
            let mut sorted = true;
            for i in 0..(v.len() - 1) - p {
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                    sorted = false;
                }
            }
            if sorted {
                return;
            }
        }
    }
}

// use by quicksort
pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56394237,
            inc: 346423491,
            modulo: 23254544563,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

// use by threaded quicksort
struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 2, 3, 1];
        let res = vec![1, 2, 3, 5];
        assert_eq!(Solution::sort_array(nums), res);

        let nums = vec![5, 1, 1, 2, 0, 0];
        let res = vec![0, 0, 1, 1, 2, 5];
        assert_eq!(Solution::sort_array(nums), res);
    }
}
