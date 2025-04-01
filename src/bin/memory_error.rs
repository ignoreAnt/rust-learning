fn recursive_function(depth: usize) {

    let _array = [0u8; 1024 * 1024]; // 1 MB stack allocation

    if depth > 0 {

        recursive_function(depth - 1); // Deep recursion

    }

}

fn main() {

    let heap_data = vec![0u8; 1024 * 1024 * 1024]; // 1 GB heap allocation

    recursive_function(1000); // Deep recursion to exhaust stack

}