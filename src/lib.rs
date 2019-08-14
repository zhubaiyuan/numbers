pub fn print() {
    let numbers = [1_u8, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }
}