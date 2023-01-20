use crate::{array::*, sets::*};
mod array;
mod sets;
fn main() {
    let (ish_array, ish_array_double) = generate_arrays();
    array(&ish_array, &ish_array_double);

    let set_a = generate_set(-100, 40);
    let set_b = generate_set(-100, 70);
    sets(&set_a, &set_b);
}
fn array(ish_array: &Vec<isize>, ish_array_double: &Vec<f32>) {
    println!("Original integer array: \n{:?}", ish_array);
    println!("Original double array: \n{:?}", ish_array_double);

    println!(
        "\nAverage of integer array: {}",
        average_of_array(ish_array)
    );
    println!(
        "Average of double array: {}",
        average_of_array_double(ish_array_double)
    );
    println!(
        "\nSmallest element of the integer array: {}",
        smallest_element_of_array(ish_array)
    );
    println!(
        "\nReverse array of the integer array: {:?}",
        reverse(ish_array)
    );
    println!(
        "\nAll even elements of the integer array: {:?}",
        return_even(ish_array)
    );
}

fn sets(array_a: &Vec<isize>, array_b: &Vec<isize>) {
    println!("\nOriginal set A: {:?}", array_a);
    println!("Original set B: {:?}", array_b);
    let mut cloned_a = array_a.clone();
    println!(
        "\nUnion of set A and B: {:?}",
        union(&mut cloned_a, array_b)
    );
    println!(
        "\nIntersect of set A and B: {:?}",
        intersect(array_a, array_b)
    );
    println!("\nComplement of set A: {:?}", complement(array_a));
    println!("\nCardinality of set A: {:?}", cardinality(array_a));
    println!(
        "Cardinality of set A and B: {:?}",
        cardinality_of_union(&mut cloned_a, array_b)
    );
}
