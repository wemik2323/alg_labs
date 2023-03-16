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
    for outter in (0..arr.len()).rev() {
        let mut sorted = true;

        for inner in 0..outter {
            if arr[inner] > arr[inner + 1] {
                arr.swap(inner, inner + 1);
                sorted = false;
            }
        }

        if sorted {
            break;
        }
    }
}

#[inline]
pub fn shell_sort(arr: &mut [i32]) {
    let mut dist = arr.len() / 2;
    let mut j;

    while dist > 0 {
        for i in dist..arr.len() {
            j = i;
            while j >= dist && arr[j - dist] > arr[j] {
                arr.swap(j - dist, j);
                j -= dist;
            }
        }
        dist /= 2;
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

    #[test]
    fn shell_sort_works() {
        let mut v1 = vec![3, -1, 12, 0, 2, 20];
        let mut v2 = v1.clone();

        shell_sort(&mut v1);
        v2.sort();

        assert_eq!(v1, v2);
    }
}
