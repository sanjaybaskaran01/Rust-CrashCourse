// Vectors are resizeable Arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("Before Change:\t{:?}", numbers);
    
    // Re-assign Value
    numbers[2] = 10;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Get single character
    println!("After Change:\t{:?}", numbers);
    println!("in index 2:{}", numbers[2]);

    // Get Vector length
    println!("Vector length:{}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice : {:?}", slice);

    // Loop through vector values
    for num in numbers.iter(){
        println!("Number:{}",num);
    }

    // Loop and mutate values
    for num in numbers.iter_mut(){
        *num*=2;
    }
    println!("Numbers Vec:{:?}",numbers);
    
}
