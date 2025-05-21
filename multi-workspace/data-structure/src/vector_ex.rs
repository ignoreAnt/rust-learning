use std::io::{self, BufRead};

/// Problem: Create an empty vector to store favorite colors (as Strings).
/// Add three colors using push(). Print the final list of colors and its length.
// Why Vector? Starts empty and grows as elements are added.
// Concepts: Vec::new(), push(), len(), iteration (for), String type.
pub fn collect_and_print_colors() {
    let mut colors = Vec::new();

    colors.push("red".to_string());
    colors.push("green".to_string());
    colors.push("blue".to_string());

    println!("{:?}", colors);
    println!("Length: {}", colors.len());
    for color in &colors {
        println!("{}", color);
    }

    colors.iter_mut().for_each(|color| {
        *color = color.to_uppercase();
    });

    println!("{:?}", colors);
}

///Problem: Simulate adding items to a shopping list. Start with an empty vector. Add "milk", "bread", and "eggs". Then, remove the last item added. Print the final list.
// Why Vector? The list changes size as items are added and removed.
// Concepts: Vec::new(), push(), pop(), println! with {:?}.
pub fn manage_shopping_list() {
    let mut cart = vec!["milk".to_string(), "bread".to_string()];
    println!(
        "Initial capacity: {:?} Length: {}",
        cart.capacity(),
        cart.len()
    );

    cart.push("eggs".to_string());

    println!(
        "Initial capacity: {:?} Length: {}",
        cart.capacity(),
        cart.len()
    );

    cart.pop();
    println!(
        "Initial capacity: {:?} Length: {}",
        cart.capacity(),
        cart.len()
    );
}

// Problem: Ask the user to enter exactly 5 numbers. Store these numbers in a vector as they are entered. Calculate and print the sum of the numbers in the vector.
// Why Vector? Even though the count (5) is fixed here, this demonstrates dynamically building a collection piece by piece, a common vector use case. An array could also work, but vector shows the building process.
// Concepts: Vec::new(), push(), loops (for or while), basic I/O (std::io), parsing input (parse), iter(), sum().
pub fn read_five_numbers_and_sum() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut nums: Vec<i32> = Vec::new();

    let mut buffer = String::new();
    for i in 0..5 {
        buffer.clear();
        handle.read_line(&mut buffer)?;

        let num: i32 = buffer.trim().parse().unwrap_or_default();
        println!("Inserting Number: {} = {}", i, num);
        nums.push(num);
    }
    let sum: i32 = nums.iter().sum();
    println!("Sum: {}", sum);
    Ok(())
}
