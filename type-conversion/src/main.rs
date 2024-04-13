fn main() {
    let x1: u64 = 4_294_967_296;
    let y1 = x1 as u32;

    if x1 == y1 as u64 {
        println!("x1({}) equals y1({})", x1, y1);
    } else {
        println!("x1({}) doesn't equal y1({})", x1, y1);
    }

    let x2: u64 = 4_294_967_296;
    let y2: u128 = x2.into();

    if x2 == y2 as u64 {
        println!("x2({}) equals y2({})", x2, y2);
    } else {
        println!("x2({}) doesn't equal y2({})", x2, y2);
    }

    let x3: u64 = 4_294_967_296;
    let y3: u32 = x3.try_into().expect("Failed to convert from u64 to u32");

    if x3 == y3 as u64 {
        println!("x3({}) equals y3({})", x3, y3);
    } else {
        println!("x3({}) doesn't equal y3({})", x3, y3);
    }
}
