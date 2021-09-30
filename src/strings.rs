// Primitive strings
//String = Growable heap allocated data structure

pub fn run(){
    let mut hello = String::from("Hello ");
    // Get Length
    println!("length={}",hello.len());

    // Push char
    hello.push('W');

    // Pushing string
    hello.push_str("orld");

    // Capacity in bytes
    println!("Capacity: {}",hello.capacity());

    // is empty
    println!("is empty: {}",hello.is_empty());

    // Contains
    println!("Contains 'World' {}",hello.contains("World"));

    // Replace
    println!("Replace: {}",hello.replace("World","There!"));

    // Loop through string by whitespace

    for word in hello.split_whitespace(){
        println!("{}",word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());

    println!("{}",s);

    println!("{}",s.capacity());



}