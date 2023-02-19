#[inline]
pub fn selection_sort(arr: &mut [i32]) {
    for iter in 0..arr.len() {
        let mut index_of_min = arr[iter..]
            .iter()
            .enumerate() // даёт отступ от `iter`
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(i, _)| i)
            .unwrap();
        index_of_min += iter; // восстановление до полного индекса

        arr.swap(iter, index_of_min);
    }
}

#[inline]
pub fn bubble_sort(arr: &mut [i32]) {
    use std::cmp::Ordering;

    for outter in (0..arr.len()).rev() {
        let mut sorted = true;

        for inner in 0..outter {
            match arr[inner].cmp(&arr[inner + 1]) {
                Ordering::Greater => {
                    arr.swap(inner, inner + 1);
                    sorted = false;
                }
                _ => (),
            }
        }

        if sorted {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sort_works() {
        let mut v1 = vec![3, -1, 12, 0, 2, 20];
        let mut v2 = v1.clone();

        selection_sort(&mut v1);
        v2.sort();

        assert_eq!(v1, v2);
    }

    #[test]
    fn bubble_sort_works() {
        let mut v1 = vec![3, -1, 12, 0, 2, 20];
        let mut v2 = v1.clone();

        bubble_sort(&mut v1);
        v2.sort();

        assert_eq!(v1, v2);
    }
}
