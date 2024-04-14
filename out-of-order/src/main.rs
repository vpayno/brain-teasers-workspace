use std::cmp::Ordering::Less;

fn main() {
    // insert order is preserved
    let mut floats = vec![3.1, 1.2, 4.5, 0.3];

    println!();
    println!("unsorted floats: {:#?}", floats);

    // E: the trait bound `{float}: std::cmp::Ord` is not satisfied: the trait `std::cmp::Ord` is not implemented for `{float}`
    // floats.sort();

    println!();
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("sorted floats: {:#?}", floats);

    println!();
    floats.reverse();
    println!("reversed floats: {:#?}", floats);

    println!();
    let mut floats2 = vec![3.1, 1.2, 4.5, 0.3, std::f32::INFINITY, std::f32::NAN];

    println!();
    println!("unsorted floats2: {:#?}", floats2);

    // treats invalid values as Less
    println!();
    floats2.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
    println!("sorted floats2: {:#?}", floats2);
}
