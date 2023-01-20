use rand::Rng;

pub fn generate_arrays() -> (Vec<isize>, Vec<f32>) {
    //Generate arrays
    let mut rng = rand::thread_rng();
    let mut random_array_length: usize = rng.gen_range(0..100);
    let mut ish_array: Vec<isize> = Vec::new();
    let mut ish_array_double: Vec<f32> = Vec::new();

    for x in 0..random_array_length {
        let random_integer_a = rng.gen::<i32>() as isize;
        let random_float = rng.gen_range(0.0..100.0);
        ish_array.push(random_integer_a);
        ish_array_double.push(random_float);
    }
    (ish_array, ish_array_double)
}
pub fn average_of_array(array: &Vec<isize>) -> isize {
    let mut sum: isize = 0;
    for x in array {
        sum += x;
    }
    return (sum / array.len() as isize).try_into().unwrap();
}
pub fn average_of_array_double(array: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for x in array {
        sum += x;
    }
    return (sum / array.len() as f32) as f32;
}

pub fn smallest_element_of_array(array: &Vec<isize>) -> isize {
    let mut smallest: isize = array[0];
    for x in array {
        if (*x < smallest) {
            smallest = *x;
        }
    }
    return smallest;
}
pub fn reverse(array: &Vec<isize>) -> Vec<isize> {
    let mut rev_array: Vec<isize> = Vec::new();
    for x in array.iter().rev() {
        rev_array.push(*x);
    }
    rev_array
}
pub fn return_even(array: &Vec<isize>) -> Vec<isize> {
    let mut v: Vec<isize> = Vec::new();
    for x in array {
        match *x % 2 {
            0 => v.push(*x),
            _ => continue,
        }
    }
    v
}
