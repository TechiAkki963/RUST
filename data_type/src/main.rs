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

    println!("You are handsome: {} and also Bodybuilder: {}", is_handsome,is_bodybuilder);

    let age: i32 = 21;
    let is_mature = age > 18;
    println!("{is_mature}");
    println!(" Is eleigible to drive : {}", is_mature);
    println!("Method Positive : {} Method Negative : {}",age.is_negative(),age.is_positive());

    // * Boolean Inversion
    let can_see_r_rated_movie = age >= 17;
    let cannot_see_r_rated_movie =  !can_see_r_rated_movie;
    println!("Your age {}, Eligibility to watch R rated movie : {}", age,can_see_r_rated_movie);

    // *Equality and Inequality
    


}
