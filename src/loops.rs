// Loops - to iterate until a condition is met

pub fn run() {
    let mut count = 1;

    // Infinite loop
    // loop{
    //     count+=1;
    //     println!("{}",count);
    //     if count==20{
    //         break;
    //     }
    // }

    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz!")
        } else if count % 3 == 0 {
            println!("Fizz!");
        } else if count % 5 == 0 {
            println!("Buzz!");
        } else {
            println!("{}", count);
        }
        //Inc
        count += 1;
    }

    // for range loop
    // for num in 1..100 {
    //     if num%15==0{
    //     println!("FizzBuzz!")
    //     }else if num%3==0{
    //         println!("Fizz!");
    //     }else if num%5==0{
    //         println!("Buzz!");
    //     }else{
    //         println!("{}",num);
    //     }
    // }
}
