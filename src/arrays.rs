// Arrays fixed list where elements are of same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];
    println!("Before Change:\t{:?}", numbers);
    numbers[2] = 10;
    // Get single character
    println!("After Change:\t{:?}", numbers);
    println!("in index 2:{}", numbers[2]);

    // Get array length
    println!("Array length:{}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice : {:?}",slice);
    println!("{}",slice==numbers);

}
