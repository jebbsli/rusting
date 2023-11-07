enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let ci = Coin::Penny;
    let v = value_in_cents(ci);
    println!("the v is {}", v);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penney!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }    
}
