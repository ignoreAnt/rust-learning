mod positive_numbers_sum;

fn main() {

    let a = 10; // i32 on stack

    let b = 20; // i32 on stack
    
    let c = 30;

    let sum = add(a, &b); // Call `add` function

    let s = String::from("hello");
    let s = "hello";
    
    println!("{}", sum);
    afunc();
}

fn add(x: i32, y: &i32) -> i32 {

    x + y // Result stored in stack frame of `add`

}

fn afunc(){
    println!("afunc");
    bfunc()
}

fn bfunc(){
    println!("bfunc");
    cfunc()
}

fn cfunc(){
    println!("cfunc");
}