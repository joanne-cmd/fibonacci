fn main() {
    // Example 1: Using your exact logic but collecting results
    let mut y = 0;
    let mut x = 1;
    let mut results = vec![0, 1]; // Start with first two Fibonacci numbers
    
    // Generate 8 more Fibonacci numbers (total of 10)
    for _ in 0..8 {
        let i = y + x;
        y = x;
        x = i;
        results.push(x);
    }
    
    // Print in block format
    print!("[");
    for (index, value) in results.iter().enumerate() {
        print!("{}", value);
        if index < results.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}
