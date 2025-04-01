fn main() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    lazy_caterer.contains(&12);
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35); // Insert 35 at index 3
    v.remove(1);     // Remove element at index 1
    assert_eq!(v, vec![10, 30, 35, 40, 50]);

    let mut v = vec![10, 20, 30, 40, 50];
    v.push(60); // Push 60 to the end of the vector
    v.pop();    // Pop the last element from the vector
    assert_eq!(v, vec![10, 20, 30, 40, 50]);

    let mut v = vec![10, 20, 30, 40, 50];
    v.reverse(); // Reverse the vector in-place in place
    assert_eq!(v, vec![50, 40, 30, 20, 10]);
    
    let mut v = vec![10, 20, 30, 40, 50];
    v.sort(); // Sort the vector in-place
    assert_eq!(v, vec![10, 20, 30, 40, 50]);
    
    let mut v = vec![10, 20, 30, 40, 50];
    v.sort_by(|a, b| b.cmp(a)); // Sort the vector in-place
    assert_eq!(v, vec![50, 40, 30, 20, 10]);
    
    let mut v = vec![10, 20, 30, 40, 50];
    v.sort_unstable(); // Sort the vector in-place
    assert_eq!(v, vec![10, 20, 30, 40, 50]);
    
    let mut v = vec![10, 20, 30, 40, 50];
    v.sort_unstable_by(|a, b| b.cmp(a)); // Sort the vector in-place
    assert_eq!(v, vec![50, 40, 30, 20, 10]); 
    
    let mut v = vec![10, 20, 30, 40, 50];

    let speech = "\"Ouch!\\\" said the well.\n";
    println!("{}", speech);

}