fn main() {
    if true {
        println!("True Output");
    }

    if false {
        println!("False Output")
    }

    let season: &str = " ";

    if season == "summer" {
        println!("It's Summer, Clouds are clear");
    } else if season == "winter" {
        println!("It's Winter, I am freezing!!!");
    } else if season == "rainy" {
        println!("Oooooh its thundering")
    } else if season == "spring" {
        println!("W@@@@W! flower blooming!!!");
    } else {
        println!("Suprisingly raining......");
    }

    even_or_odd(20);
    even_or_odd(17);


    // *Match Statement

    let evaluation: bool = true;

    let value: i32 = match evaluation {
        true => 20,
        false => 40,
    };
    println!("{value}");


    // *Match Statement multiple values
    let number: i32 = 8;

    match number{
        value if value % 2 == 0 => println!("{value} is even number"),
        x if value % 2 != 0 => println!("{x} is odd number"),
        _ => unreachable!(),  //   **Macro
    }


    // ** loops and break

    let seconds = 10;
    
}

fn even_or_odd(number: i32) {
    let result: &str = if number % 2 == 0 { "even" } else { "odd" };
    
    println!("The number is {result}");
}