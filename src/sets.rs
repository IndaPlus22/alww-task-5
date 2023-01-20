use std::collections::HashSet;

pub fn generate_set(mut min: i32, mut max: i32) -> Vec<isize> {
    let mut set = Vec::<isize>::new();
    if (min < 0) {
        min = 0;
    }
    if (max > 100) {
        max = 100;
    }
    if (min > max) {
        return set;
    }
    for x in min..max {
        set.push(x as isize)
    }
    set
}

fn hashset(data: &[isize]) -> HashSet<isize> {
    HashSet::from_iter(data.iter().cloned())
}
pub fn union(a: &mut Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    a.extend(b);
    a.to_owned()
}
pub fn intersect(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    let hash_a = hashset(a);
    let hash_b = hashset(a);
    hash_a.intersection(&hash_b).map(|i| *i).collect()
}

pub fn complement(a: &Vec<isize>) -> Vec<isize> {
    let hash = hashset(a);
    let mut b: HashSet<isize> = HashSet::new();
    for x in 0..99 {
        b.insert(x);
    }
    hash.difference(&b).map(|i| *i).collect()
}

pub fn cardinality(a: &Vec<isize>) -> usize {
    hashset(a).iter().map(|i| *i).collect::<Vec<_>>().len()
}

pub fn cardinality_of_union(a: &mut Vec<isize>, b: &Vec<isize>) -> usize {
    a.extend(b);
    hashset(a).iter().map(|i| *i).collect::<Vec<_>>().len()
}
