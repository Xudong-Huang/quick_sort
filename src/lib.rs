#[macro_use]
extern crate may;

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

pub fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    // for each coroutine the min size of each job
    if v.len() <= 0x1000 {
        return v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);

    join!(quick_sort(lo), quick_sort(hi));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Send + Ord>(v: &[T]) -> bool {
        (1..v.len()).all(|i| v[i - 1] <= v[i])
    }

    #[test]
    fn it_works() {
        let mut v = vec![8, 2, 9, 6, 5, 0, 1, 4, 3, 7];
        quick_sort(&mut v);
        assert!(is_sorted(&v));
    }
}
