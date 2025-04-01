use std::any::type_name;

fn main() {
    let mut x = 42;
    let ref_x: &i32 = &x; // Borrow a shared reference to x
    let ref_x1: &i32 = &x; // Borrow a mutable reference to x
    // let ref_x2: &mut i32 = &mut x; // Borrow a mutable reference to x
    println!("Value of x: {}", *ref_x); // Dereference to access the value
    println!("Value of x: {}", *ref_x1); // Dereference to access the value
    println!("Address of x: {:p}", ref_x);
    println!("Address of x: {:p}", ref_x1);
    println!("Address of x: {:p}", &x);
    print_type_of(&x);
    print_type_of(&ref_x);
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}