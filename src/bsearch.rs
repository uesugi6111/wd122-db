use std::cmp::Ordering::{self, Equal, Greater, Less};

pub fn binary_search_by<F>(mut size: usize, mut f: F) -> Result<usize, usize>
where
    F: FnMut(usize) -> Ordering,
{
    let mut left = 0;
    let mut right = size;
    while left < right {
        let mid = left + size / 2;

        match f(mid) {
            Less => {
                left = mid + 1;
            }
            Equal => {
                return Ok(mid);
            }
            Greater => {
                right = mid;
            }
        }
        size = right - left;
    }
    Err(left)
}
