#[inline]
pub fn linear_search(arr: &[u8], substr: &[u8]) -> Option<usize> {
    for (ind, win) in arr.windows(substr.len()).enumerate() {
        if win == substr {
            return Some(ind);
        }
    }

    None
}

#[inline]
pub fn rabin_search(arr: &[u8], substr: &[u8]) -> Option<usize> {
    let hash = |s: &[u8]| s.first().unwrap() + s.last().unwrap();

    let substr_sum = hash(substr);
    for (ind, win) in arr.windows(substr.len()).enumerate() {
        let sm = hash(win);
        if sm == substr_sum {
            if win == substr {
                return Some(ind);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_search_works() {
        let s = "qpoueangygnzlhtahgegbbxlotw";

        assert_eq!(linear_search(s.as_bytes(), "tahge".as_bytes()), Some(14));
        assert_eq!(linear_search(s.as_bytes(), "rand".as_bytes()), None);
    }

    #[test]
    fn rabin_search_works() {
        let s = "qpoueangygnzlhtahgegbbxlotw";

        assert_eq!(rabin_search(s.as_bytes(), "tahge".as_bytes()), Some(14));
        assert_eq!(rabin_search(s.as_bytes(), "rand".as_bytes()), None);
    }
}
