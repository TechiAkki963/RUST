fn main() {
    make_cream("Mango");
    make_cream("Chocolate");
    make_cream("Coconut");
    base_option("Bucket");
    serve(3, "chocochips");
    serve_icecream();

    let result =square(5);
    println!("The square of 5 : {result}");

    let result =square(25);
    println!("The square of 25: {result}");
}


fn make_cream(fruit: &str){
    println!("Making Fruit Creams using {fruit}");
}

fn base_option(base: &str){
    println!("Base Option : {base}");
}

fn serve(scoops: i8, toppings: &str){
    println!("No. of Scoops: {scoops}, Toppings:{toppings}")
}

fn serve_icecream(){
    println!("Serve the icecream");
}

// *Return Value
fn square(number: i32) -> i32{
    return number*number;
}
