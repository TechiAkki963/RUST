use std::result;

fn main() {
    make_cream("Mango");
    make_cream("Chocolate");
    make_cream("Coconut");
    base_option("Bucket");
    serve(3, "chocochips");
    serve_icecream();

    let result1 = square(5);
    println!("The square of 5 : {result1}");

    let result2 = square(25);
    println!("The square of 25: {result2}");

    let result3 = rectangle(5, 3);
    println!("Area of Rectangle: {result3} ccm.");

    let result4: bool = is_even(15);
    println!("The number is even: {result4}");

    let result5: i32 = cube(3);
    println!("The cube : {result5}");
}

fn make_cream(fruit: &str) {
    println!("Making Fruit Creams using {fruit}");
}

fn base_option(base: &str) {
    println!("Base Option : {base}");
}

fn serve(scoops: i8, toppings: &str) {
    println!("No. of Scoops: {scoops}, Toppings:{toppings}")
}

fn serve_icecream() {
    println!("Serve the icecream");
}

// *Return Value
fn square(number: i32) -> i32 {
    return number * number;
}

fn rectangle(width: u32, height: u32) -> u32 {
    return width * height;
}

fn is_even(number: i32) -> bool {
    return number % 2 == 0;
}

// *Implicit

fn cube(n: i32) -> i32 {
    n * n * n
}
//Implicit return value - the last line in the scope of a function without ';'
