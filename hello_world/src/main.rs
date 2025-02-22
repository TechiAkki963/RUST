#![allow(unused_variables)]

type Meters = i32;

const TAX_PERCENTAGE:i8 = 10;

fn main() {
    println!("Hello from Cargo");

    println!("I am a RUST");

    let apples = 50;
    let oranges = 20 + 5;
    let _bucket = apples + oranges;

    println!("{}", oranges);
    println!("This years, My garden has {apples} apples and {oranges} oranges");
    println!("{} apples", apples - 10);

    println!(
        "I saw {} apples and {} oranges in my garden, I was Surprised to see {} apples",
        apples, oranges, apples
    );

    println!(
        "I saw {0} apples and {1} oranges in my garden, I was Surprised to see {0} apples",
        apples, oranges
    );

    // *Mutable && Immutable

    let mut gym_reps = 15;
    println!("I do {} bench press in a single set", gym_reps);

    gym_reps = 20;

    println!("Now I do {} bench press in a single set", gym_reps);

    //  *Variable Shadowing

    let _quantity_of_rice = "100.50";
    let _quantity_of_rice=200.15;
    let mut quantity_of_rice=15;
    quantity_of_rice=18;
    println!("I bought {} kgs of rice ",quantity_of_rice);



    // *Scope

    let price_of_coffee = 50;

    {   
        let price_of_tea=100;
        println!("You have to pay Rs.{} for this Tea(inner)",price_of_tea);
        println!("You have to pay Rs.{} for this coffee",price_of_coffee);

        let price_of_coffee= 75;
        println!("You have to pay Rs.{} for the coffee", price_of_coffee);
        // This syntax does is creates a sepearte independant coffee variable within this scope
        // So the output now will be 75.
    }
    // println!("You have to pay Rs.{} for this Tea(outer)",price_of_tea); (! Not found in this scope)



    // *CONSTANTS | const
    
    // TAX_RATE is declared outside the fn main as it can be used in any file level / scope
    // const Naming convention , All the letters should always be Capital
    let current_salary = 1000;
    println!("Candidates current salaty:{current_salary}, so he has to {TAX_PERCENTAGE} as tax");


    // *Type Alias
    // Type Alias is decalred outside the fn main as it can be in any file level
    // Type Alias Naming convention , the first word should always be Capital

    // let length_of_track:i32 = 1600;
    let length_of_track: Meters= 1600;  
    println!("The length of the track {} meters",length_of_track);

    // *Compiler Directives

    // Compiler Directives is an annotation that tells the compiler how to parse the source sode
    // Compiler Directive can be applied to a line , a function or entire files
    // if applying to entire use "!"  #![allow(unused_variables)]

    // #[allow(unused_variables)]
    let number_of_mangoes = 24;

    // #[allow(unused_variables)]
    let number_of_bananas = 12;

}
