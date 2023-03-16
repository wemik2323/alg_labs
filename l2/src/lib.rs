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
        if sm == substr_sum && win == substr {
            return Some(ind);
        }
    }

    None
}

#[inline]
pub fn knuth_search(arr: &[u8], substr: &[u8]) -> Option<usize> {
    let mut d = vec![0; substr.len()];
    let mut j = 0;
    for i in 1..substr.len() {
        while j > 0 && substr[j] != substr[i] {
            j = d[j - 1];
        }
        if substr[j] == substr[i] {
            j += 1;
        }
        d[i] = j;
    }

    j = 0;
    for i in 1..arr.len() {
        while j > 0 && substr[j] != arr[i] {
            j = d[j - 1];
        }
        if substr[j] == arr[i] {
            j += 1;
        }
        if j == substr.len() {
            return Some(i - j + 1);
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

    #[test]
    fn knuth_search_works() {
        let s = "qpoueangygnzlhtahgegbbxlotw";

        assert_eq!(knuth_search(s.as_bytes(), "tahge".as_bytes()), Some(14));
        assert_eq!(knuth_search(s.as_bytes(), "rand".as_bytes()), None);
    }
}
