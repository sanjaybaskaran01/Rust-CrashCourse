pub fn run() {
    let name = "Sanjay";
    let mut age = 19;
    println!("My name is {} and I am {}", name, age);
    age = age + 1;

    println!("My name is {} and I am {}", name, age);
    // Assign multiple characters
    let (my_name, my_age) = ("Sanjay", 19);
    println!("{} is {}", my_name, my_age)
}
