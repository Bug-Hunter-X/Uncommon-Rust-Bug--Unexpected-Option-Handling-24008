fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.pop();

    if let Some(n) = number {
        println!("The last number is {}", n);
    } else {
        println!("The vector is empty");
    }
} 