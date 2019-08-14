pub fn print() {
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }
}