pub fn print() {
    let numbers = vec![1, 2, 3, 4, 5];
    output_sequence(numbers);
}

fn output_sequence(numbers: Vec<u8>) {
    for n in numbers {
        println!("{}", n);
    }
}