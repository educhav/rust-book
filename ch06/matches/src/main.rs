#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Wisconsin,
    NewYork
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;

    println!("Value of a penny: {}", value_in_cents(penny));
    println!("Value of a nickel: {}", value_in_cents(nickel));
    // println!("Value of a dime: {}", value_in_cents(dime));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // other will cover all other possible values and bind to that value so we can use it
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // we can use _ instead if we do not wish to use the value or bind to it
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // we can use () as the arm code if we wish to do nothing in the case where it not 3 or 7
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    }


    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max of the config is: {}", max),
        _ => ()
    }

    // Use if let as a shorthand
    // if let <pattern> = <expr>
    // If the pattern matches, execute the code in the curly braces, otherwise () -- do nothing
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.


    // Can use else in if-let statement

    // let mut count = 0;
    // match dime {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    let mut count = 0;
    if let Coin::Quarter(state) = dime {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }



}

// Matches must be exhaustive: cover all cases, will fail to compile if not exhaustive
fn plus_one(option: Option<i32>) -> Option<i32> {
    match option {
        None => None,
        Some(num) => Some(num+1)
    }
}


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
