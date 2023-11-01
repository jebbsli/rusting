fn main() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("the value of c is: {c}");

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let first_ = tup2.0;
    let second_ = tup2.1;
    let third_ = tup2.2;
    println!("{first_} {second_} {third_}");
}
