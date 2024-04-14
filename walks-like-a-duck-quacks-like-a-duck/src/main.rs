fn double_it(n: u64, _: i32) -> u64 {
    n * 2
}

fn triple_it(n: u128) -> u128 {
    n * 3
}

fn main() {
    let one: i32 = 1;

    // 1st arg is used to define arg type, second arg gets discarded
    let d = double_it(one as _, 3);

    // 1st arg is used to define arg type, 2nd arg was a distraction
    let t = triple_it(one as _);

    println!("   one: {}, {:?}", one, std::any::type_name_of_val(&one));
    println!("double: {}, {:?}", d, std::any::type_name_of_val(&d));
    println!("triple: {}, {:?}", t, std::any::type_name_of_val(&t));
}
