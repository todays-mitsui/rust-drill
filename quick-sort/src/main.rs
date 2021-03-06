use std::cmp::Ordering;

fn sort<T: Ord + Clone>(xs: Vec<T>) -> Vec<T> {


    if xs.len() == 0 { return Vec::new(); }

    let pivot = &xs[0];

    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut greater = Vec::new();

    for x in &xs {
        match x.cmp(&pivot) {
            Ordering::Less => less.push((*x).clone()),
            Ordering::Equal => equal.push((*x).clone()),
            Ordering::Greater => greater.push((*x).clone()),
        }
    }

    let mut result = Vec::new();

    result.append(&mut sort(less));
    result.append(&mut equal);
    result.append(&mut sort(greater));

    return result;
}

fn partition<T: Ord + Clone>(xs: Vec<T>, pivot: T2, left: usize, right: usize) {
    while
}

#[test]
fn test_sort() {
    assert_eq!(
        sort(vec![2, 5, 3, 1, 4]),
        vec![1, 2, 3, 4, 5]
    );
}

fn main() {
}
