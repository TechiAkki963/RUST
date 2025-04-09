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

    match number {
        value if value % 2 == 0 => println!("{value} is even number"),
        x if value % 2 != 0 => println!("{x} is odd number"),
        _ => unreachable!(), //   **Macro
    }

    // ** loops and break

    let mut seconds = 10;

    loop {
        if seconds == 0 {
            println!("BOOOOOOOOOM!!!");
            break;
        }

        println!("{seconds} to have a blastoff...");
        // seconds = seconds - 1;
        seconds -= 1;
    }

    // **Loop and Continue keyword

    let mut timer = 21;

    loop {
        if timer == 0 {
            println!("Boom!!!");
            break;
        }

        if timer % 2 == 0 {
            println!("{timer} is even number. Skipping 3 secs ....");
            timer -= 3;
        }

        println!("{timer} seconds to have a blast");
        timer -= 1;
    }

    // While

    let mut minute = 60;

    while minute > 0 {
        if minute % 2 == 0 {
            println!("{minute} secs ...Skipping 3 seconds (in while)");
            minute -= 3
        }

        println!("{minute} secs to go Boom (in While)");
        minute -= 1
    }

    println!("Boom!!! (In While)");

    // ** Recurssion
    countdown(5);
}

fn even_or_odd(number: i32) {
    let result: &str = if number % 2 == 0 { "even" } else { "odd" };

    println!("The number is {result}");
}

// ** Recursion
// Recursion is when a function call itself
fn countdown(secs: i32) {
    if secs == 0 {
        println!("{secs} Blastoff!!!")
    } else {
        println!("{secs} seconds for Launch...");
        countdown(secs - 1);
    }
}
