fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.pop(); //This line is the bug!

    match number {
        Some(n) => println!("The last number is {}", n),
        None => println!("The vector is empty"),
    }
}