enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}

fn main(){
    let random_coin = Coin::Nickel;
    println!("The value of Random Coin is {}", value_in_cents(random_coin));
}