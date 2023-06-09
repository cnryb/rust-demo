fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for number in [1, 2, 3] {
        println!("{number}!");
    }

    let result = loop {
        break 2 * 2;
    };
    println!("The result is {result}");

    'l1: loop {
        println!("Hello, world! in l1");
        loop {
            println!("Hello, world! in inner loop");
            break 'l1;
        }
    }
}
