fn main() {

println!("{}",color_to_number("red"));
println!("{}",color_to_number("green"));
println!("{}",color_to_number("blue"));
println!("{}",color_to_number("purple"));
}

// fn color_to_number(color: &str) {
// if color == "red" {
// println!("{}", 1);
// } else if color == "green" {
// println!("{}", 2);
// } else if color == "blue" {
// println!("{}", 3);
// } else {
// println!("{}", 0);
// }
// }

// Using match statement

fn color\*to_number(color: &str) -> i32 {
match color {
"red" =>1,
"green" => 2,
"blue" => 3,

- => 0,
  }
  }

// Factorial

fn factorial (number: i32) -> i32 {
let mut product: i32 = 1;
let mut count: i32 = number;

    loop {
        if count > 0 {
            product = product * count;
            count -= 1;
        }else{
            return product;
        }
    }

}

fn main(){
println!("{}", factorial(5));
}
