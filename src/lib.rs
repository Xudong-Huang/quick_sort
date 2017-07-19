#[macro_use]
extern crate may;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn partition_v1<T: PartialOrd + Send>(v: &mut [T], pivot: T) -> usize {
    let mut j = v.len() - 1;
    let mut i = 0;

    while i < j {
        if v[i] >= pivot {
            v.swap(i, j);
            j -= 1;
            continue;
        }
        i += 1
    }

    if v[i] < pivot {
        i += 1;
    }

    i
}


fn partition_v2<T: PartialOrd + Send>(v: &mut [T], pivot_1: T, pivot_2: T) -> (usize, usize) {
    let mut j = v.len() - 1;
    let mut i = 0;
    let mut p = 0;

    let (min, max) = if pivot_1 > pivot_2 {
        (pivot_2, pivot_1)
    } else {
        (pivot_1, pivot_2)
    };

    while p <= j {
        if v[p] <= min {
            v.swap(i, p);
            i += 1;
        }

        if v[p] >= max {
            v.swap(p, j);
            if j == 0 {
                j = 0
            } else {
                j -= 1;
            }
            continue;
        }

        p += 1
    }

    if v[j] < max {
        j += 1;
    }

    // if v[i] <= min {
    //     i += 1;
    // }

    if i > j {
        j = i;
    }

    (i, j)
}

pub fn quick_sort_v1<T: PartialOrd + Send + Copy>(v: &mut [T]) {
    // for each coroutine the min size of each job
    let len = v.len();
    if len <= 0x1000 {
        return v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }


    // if len <= 1 {
    //     return;
    // }

    // if len == 2 {
    //     if v[0] > v[1] {
    //         v.swap(0, 1);
    //     }
    //     return;
    // }

    // if len == 3 {
    //     return v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // }

    let mut i = 0;
    let pivot_1 = v[len - 1];
    let mut pivot_2 = v[0];

    while pivot_2 == pivot_1 {
        i += 1;
        if i == len {
            // all the elements are the same value, just return
            return;
        }
        pivot_2 = v[i];
    }

    let (lo, hi) = partition_v2(v, pivot_1, pivot_2);
    let (r1, r2) = v.split_at_mut(lo);
    let (r2, r3) = r2.split_at_mut(hi - lo);

    // let (mut min, mut max) = if pivot_1 > pivot_2 {
    //     (pivot_2, pivot_1)
    // } else {
    //     (pivot_1, pivot_2)
    // };

    // if r1.len() > 0 && !r1.iter().all(|v| *v <= min) {
    //     min = pivot_1;
    //     panic!("r1 failed");
    // }

    // if r2.len() > 0 && !r2.iter().all(|v| *v > min && *v < max) {
    //     min = pivot_1;
    //     panic!("r2 failed");
    // }

    // if r3.len() > 0 && !r3.iter().all(|v| *v >= max) {
    //     max = pivot_2;
    //     panic!("r3 failed");
    // }

    // quick_sort_v1(r1);
    // quick_sort_v1(r2);
    // quick_sort_v1(r3);
    join!(quick_sort_v1(r1), quick_sort_v1(r2), quick_sort_v1(r3));
}

pub fn quick_sort<T: PartialOrd + Send + Copy>(v: &mut [T]) {
    // for each coroutine the min size of each job
    if v.len() <= 0x1000 {
        return v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    // if v.len() <= 1 {
    //     return;
    // }

    // let last = v.len() - 1;
    // let pivot = v[last];
    // let mid = partition_v1(v, pivot);
    let mid = partition(v);

    let (lo, hi) = v.split_at_mut(mid);

    // quick_sort(lo);
    // quick_sort(hi);
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
        assert_eq!(is_sorted(&v), true);
    }
}
