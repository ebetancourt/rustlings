// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    // first solution that worked
    // let mut vec1 = fill_vec(vec0.clone());

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // all of these worked
    // let mut vec = vec.to_vec();
    // let mut vec = vec.clone();
    let mut vec = Vec::from(vec.as_slice());

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
