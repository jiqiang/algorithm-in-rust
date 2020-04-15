extern crate rand;

pub mod union_find;

use rand::Rng;

pub fn fisher_yates_shuffle<T>(items: &mut Vec<T>) {
    let mut m = items.len();
    let mut rng = rand::thread_rng();
    while m > 1 {
        m -= 1;
        let r = rng.gen_range(0, m);
        items.swap(r, m);
    }
}

pub fn gcd(p: usize, q: usize) -> usize {
    if q == 0 {
        return p;
    }
    let r: usize = p % q;
    gcd(q, r)
}

pub fn binary_search(key: i32, mut a: Vec<i32>) -> Option<usize> {
    use std::cmp::Ordering;
    a.sort();
    let mut lo: usize = 0;
    let mut hi: usize = a.len() - 1;
    while lo <= hi {
        let mid: usize = lo + (hi - lo) / 2;
        let mid_value = match a.get(mid) {
            Some(value) => value,
            None => return None,
        };
        match key.cmp(&mid_value) {
            Ordering::Less => hi = mid - 1,
            Ordering::Greater => lo = mid + 1,
            Ordering::Equal => return Some(mid),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fisher_yates_shuffle_works() {
        let mut items: Vec<i32> = (1..=10).collect();
        let old_len = items.len();
        fisher_yates_shuffle(&mut items);
        let new_len = items.len();
        assert_eq!(old_len, new_len);
        assert_ne!((1..=10).collect::<Vec<i32>>(), items);
    }

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(0, 8), 8);
    }

    #[test]
    fn binary_search_works() {
        assert_eq!(binary_search(3, vec![1, 2, 3, 4, 5, 6]), Some(2));
        assert_eq!(binary_search(1, vec![1, 2, 3, 4, 5, 6]), Some(0));
        assert_eq!(binary_search(6, vec![1, 2, 3, 4, 5, 6]), Some(5));
        assert_eq!(binary_search(2, vec![1, 3, 5, 7, 9]), None);
    }
}
