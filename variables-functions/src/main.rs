fn main() {
    const PI: f64 = 3.14;
    let x = 5;
    let mut y = 5;
    {
        let x = x as f64 * PI;
        println!("x = {}", x);
    }

    println!("x = {}", x);

    y += 1;
    println!("y = {}", y);

    println!("plus_one(y) = {}", plus_one(y));
    println!("plus_two(y) = {}", plus_two(y));

    println!("Hello, world!");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    return x + 2;
}
