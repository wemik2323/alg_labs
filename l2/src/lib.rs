#[inline]
pub fn linear_search(arr: &[i32], key: i32) -> Option<usize> {
    for (index, elem) in arr.iter().enumerate() {
        if *elem == key {
            return Some(index);
        }
    }

    None
}

#[inline]
pub fn binary_search(arr: &[i32], key: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let middle = (left + right) / 2;

        let current = arr[middle];

        use std::cmp::Ordering;
        match current.cmp(&key) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => {
                if middle == 0 {
                    return None;
                }

                right = middle - 1;
            }
            Ordering::Less => left = middle + 1,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_search_works() {
        let v = vec![3, -1, 12, 0, 2, 20];

        assert_eq!(linear_search(&v, 12), Some(2));
        assert_eq!(linear_search(&v, 13), None);
    }
    #[test]
    fn binary_search_works() {
        let v = vec![-1, 0, 2, 3, 12, 20];

        assert_eq!(binary_search(&v, 12), Some(4));
        assert_eq!(binary_search(&v, 13), None);
    }
}
