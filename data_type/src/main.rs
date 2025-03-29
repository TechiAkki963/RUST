// use core::num;

fn main() {
    let _small_integer: i8 = 32;

    let sixteen_bit_signed: i16 = -32_500;
    let sixteen_bit_unsigned: u16 = 64_000;

    println!(
        "Signed bit 16 : {} Unsigned bit 16 : {}",
        sixteen_bit_signed, sixteen_bit_unsigned
    );

    let thirty_two_bit_signed: i32 = -2_14_74_83_648;
    let thirty_two_bit_unsigned: u32 = 4_29_49_67_295;

    println!(
        "Signed bit 32: {}, Unsigned bit 32: {}",
        thirty_two_bit_signed, thirty_two_bit_unsigned
    );

    // *usize
    // *isize

    let days: usize = 55;
    let years: isize = -15_000;
    println!(
        "Dinosuar born year {} ago and Fossil was found {} days ago ",
        years, days
    );

    // *Strings and Raw String

    println!("Dear Emily \nHow are you?"); // \n - to move to next line
    println!("\t Once upon a tyne"); //  \t - to auto apply tab
    println!("Juliet said \"I love you Romeo\""); // \- to avoid "" confusion

    let filepath = r"C:\My Document\new\videos"; // r - raw string - to avoid confusion of \
    println!("{filepath}");

    // *Methods

    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_value = "    Empty Value     ";
    println!("{}", empty_value.trim());

    println!("{}", value.pow(2));

    println!("{}", value.pow(3));

    // * floating Point

    let pi: f64 = 3.14159;
    println!("The value of pi {}", pi);

    println!("{}", pi.floor()); //3
    println!("{}", pi.ceil()); //4
    println!("{}", pi.round());

    // * float format specifier

    println!("The current value of pi is {pi:.2}"); //3.14

    // * Casting type with "as" keyword

    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    println!("{miles_away_i8}");
    println!("{miles_away_u8}");

    let miles_away: f64 = 100.641239784654321213115645;
    let miles_away_f32 = miles_away as f32;
    let miles_away_i8 = miles_away as i8;
    println!("{miles_away_f32}");
    println!("{miles_away_i8}");

    // *Math Operators

    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;

    println!(
        "addition : {} , subtraction : {} , multiplication : {}",
        addition, subtraction, multiplication
    );

    let floor_division = 5 / 3;
    println!("division : {}", floor_division);

    let decimal_division = 5.0 / 3.0;
    println!("decimal division : {}", decimal_division);

    let reminder = 9 % 2;
    println!("{}", reminder);

    // *Augmented Operator
    let mut year: i32 = 2025;
    // year = year + 1;
    year += 1;
    println!("increment : {}", year);

    year -= 2;
    println!("decrement : {}", year);

    year *= 3;
    println!("multiplication : {}", year);

    year /= 5;
    println!("dividation : {}", year);

    // *Boolean

    let is_handsome = true;
    let is_bodybuilder = false;

    println!(
        "You are handsome: {} and also Bodybuilder: {}",
        is_handsome, is_bodybuilder
    );

    let age: i32 = 21;
    let is_mature = age > 18;
    println!("{is_mature}");
    println!(" Is eleigible to drive : {}", is_mature);
    println!(
        "Method Positive : {} Method Negative : {}",
        age.is_negative(),
        age.is_positive()
    );

    // * Boolean Inversion
    let can_see_r_rated_movie = age >= 17;
    #[allow(unused_variables)]
    let cannot_see_r_rated_movie = !can_see_r_rated_movie;
    println!(
        "Your age {}, Eligibility to watch R rated movie : {}",
        age, can_see_r_rated_movie
    );

    // *Equality and Inequality

    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke ");
    println!("{}", "Coke" == "Coke");

    println!("{}", 13 == 13);
    println!("{}", 13 != 13);

    println!("{}", 26.1 == 26.1);
    println!("{}", 26.1 == 26.14);

    println!("{}", 13 == 13.0 as i32);

    println!("{}", true == true);
    println!("{}", false == false);
    println!("{}", true != false);

    //  And && operator
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);

    // OR || operator
    let user_has_paid_for_subscription = false;
    let is_admin = true;
    let user_can_see_premium_content = user_has_paid_for_subscription || is_admin;
    println!(
        "Can this user view our content? : {}",
        user_can_see_premium_content
    );

    // Char
    let initial_char = 'B';
    let emoji = '💋';
    println!("{} {}", initial_char, emoji);
    println!("{} {}", initial_char.is_alphabetic(), emoji.is_alphabetic());
    println!("{} {}", initial_char.to_lowercase(), emoji.to_lowercase());
    println!("{} {}", initial_char.to_uppercase(), emoji.to_uppercase());
    println!("{} {}", initial_char.is_lowercase(), emoji.is_lowercase());
    println!("{} {}", initial_char.is_lowercase(), emoji.is_lowercase());


    // Array

    let number:[i32;7] = [1, 12, 21, 27, 36, 45, 90];
    let fruits = ["Apple", "Mango", "Banana"];
    println!("Number: {}",number.len());
    println!("List of Fruits: {}",fruits.len());
    
    let season: [&str; 5] = ["Winter", "Autumn", "Summer","Rainy","Spring"];
    println!("I hate {}",season[2]);

    let first_season = season[0];
    let third_season= season[2];
    println!("The first season {} and the Third Season {} are the main season from the list",first_season, third_season);


    // Mut Array
    let mut veges =  ["carrot", "potato", "tomato"];
    println!("Second Vege : {}",veges[1]);
    veges[1]= "Spinach";
    println!("Replaced to : {}",veges[1]);

    
    // Display Trait
    #[allow(unused_variables)]
    let games: [&str; 4] = ["Cricket", "Football", "Hockey", "Basketball"];
    println!("{}", 5);
    println!("{}",3.14);
    println!("{}", true);
    // println!("{}", games);
    // help: the trait `std::fmt::Display` is not implemented for `[&str; 4]`

    //The Display trait is for user-friendly output.
    // Basic types like integers, floats, and booleans implement Display.
    // Arrays, by default, do not implement Display.
    // The Debug trait, is used for developer output, and is very useful for viewing the contents of data structures.
    
    // **Debug Trait** :?
    println!("{:?}",games);
    // ["Cricket", "Football", "Hockey", "Basketball"]

    // Also can be written
    println!("{games:?}");
    // ["Cricket", "Football", "Hockey", "Basketball"]

    // for pretty format also
    println!("{games:#?}");
    //   [
    //     "Cricket",
    //     "Football",
    //     "Hockey",
    //     "Basketball",
    //    ]

    

    //**  The dbg! Macro
    dbg!(games);
    // [src/main.rs:245:5] games = [
    //     "Cricket",
    //     "Football",
    //     "Hockey",
    //     "Basketball",
    // ]
    dbg!(2 + 2);
    // [src/main.rs:252:5] 2 + 2 = 4
    

    // Tuple

    let employee = ("Molly", 32, 5.5, "Marketing");

    let name = employee.0;
    let age = employee.1;
    let experience = employee.2;
    let department =  employee.3;
   

    println!("Name: {name}, Age: {age}, Exp: {experience}yrs, Department: {department}");
    // Name:Molly, Age:32, Exp:5.5, Department:Marketing

    println!("{employee:#?}");    
    // (
    //     "Molly",
    //     32,
    //     "Marketing",
    //     5.5,
    // )

    let student =("Alex", 10, "A*A*A*");
    let(student_name, student_class, student_grade)=student;
    println!("Name: {student_name}, Class: {student_class}, Grade: {student_grade}");
    // Name: Alex, Class: 10, Grade: A*A*A*
    println!("{student:#?}");
    // (
    //     "Alex",
    //     10,
    //     "A*A*A*",
    // )

}
