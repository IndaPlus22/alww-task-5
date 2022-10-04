use std::{cmp, collections::HashSet};

fn main() {
    // array();
    sets();
}
fn array() {
    let array = [1, 2, 3, 4];
    println!("{}", average_of_array(array));
    println!("{}", average_of_array_double(array));
    println!("{}", smallest_element_of_array(array));
    println!("{:?}", reverse(array));
    println!("{:?}", return_even(array));
}

fn sets() {
    let a: HashSet<usize> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<usize> = [3, 4, 5, 6].iter().cloned().collect();
    println!("{:?}", union(a, b));
    let a: HashSet<usize> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<usize> = [3, 4, 5, 6].iter().cloned().collect();
    println!("{:?}", intersect(a, b));
    let a: HashSet<usize> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<usize> = [3, 4, 5, 6].iter().cloned().collect();
    println!("{:?}", complement(a, b));
    let a: HashSet<usize> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<usize> = [3, 4, 5, 6].iter().cloned().collect();
    println!("{:?}", cardinality_of_union(a, b));
}

fn average_of_array(array: [usize; 4]) -> u8 {
    let mut sum = 0;
    for x in array {
        sum += x;
    }
    return (sum / array.len()).try_into().unwrap();
}
fn average_of_array_double(array: [usize; 4]) -> f32 {
    let mut sum = 0;
    for x in array {
        sum += x;
    }
    return (sum / array.len()) as f32;
}
fn smallest_element_of_array(array: [usize; 4]) -> usize {
    return return_smallest(
        &return_smallest(&array[2], &array[3]),
        &return_smallest(&array[0], &array[1]),
    );
}
fn return_smallest(a: &usize, b: &usize) -> usize {
    match a.cmp(b) {
        cmp::Ordering::Less => return a.clone(),
        cmp::Ordering::Greater => return b.clone(),
        cmp::Ordering::Equal => return a.clone(),
    }
}
fn reverse(array: [usize; 4]) -> [usize; 4] {
    return [array[3], array[2], array[1], array[0]];
}
fn return_even(array: [usize; 4]) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    for x in array {
        match x % 2 {
            0 => v.push(x),
            _ => continue,
        }
    }
    v
}

fn union(mut a: HashSet<usize>, b: HashSet<usize>) -> HashSet<usize> {
    a.extend(&b);
    return a;
}
fn intersect(a: HashSet<usize>, b: HashSet<usize>) -> HashSet<usize> {
    return &a & &b;
}

fn complement(mut a: HashSet<usize>, b: HashSet<usize>) -> HashSet<usize> {
    a.extend(&b);
    let mut c: HashSet<usize> = HashSet::new();
    for x in 0..99 {
        c.insert(x);
    }
    return &a ^ &c;
}

fn cardinality_of_union(mut a: HashSet<usize>, b: HashSet<usize>) -> usize {
    a.extend(&b);
    return a.len();
}

fn cardinality(a: HashSet<usize>) -> usize {
    return a.len();
}
