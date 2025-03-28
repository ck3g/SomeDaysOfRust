fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 5;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}")
    }

    println!("The value of y is: {y}");

    let tup = (500, 6.4, 'A');
    let (x, y, a) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of a is: {a}");

    let x = tup.0;
    println!("The value of x is: {x}");
    let y = tup.1;
    println!("The value of y is: {y}");
    let a = tup.2;
    println!("The value of a is: {a}");
}
