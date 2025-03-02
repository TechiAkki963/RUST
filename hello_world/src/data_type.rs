fn main() {
    let _small_integer: i8 = 32;

    let sixteen_bit_signed: i16 = -32_500;
    let sixteen_bit_unsigned: u16 = 64_000;

    println!("Signed bit 16 : {} Unsigned bit 16 : {}", sixteen_bit_signed,  sixteen_bit_unsigned);

    let thirty_two_bit_signed: i32 = -2_14_74_83_648;
    let thirty_two_bit_unsigned: u32 = 4_29_49_67_295;

    println!("Signed bit 32: {}, Unsigned bit 32: {}",thirty_two_bit_signed,thirty_two_bit_unsigned );

    // *usize 
    // *isize

    let days: usize = 55;
    let years: isize = -15_000;
    println!("Dinosuar born year {} ago and Fossil was found {} days ago ",years,days);


    // *Strings and Raw String

    println!("Dear Emily \nHow are you?");   // \n - to move to next line
    println!("\t Once upon a tyne");         //  \t - to auto apply tab
    println!("Juliet said \"I love you Romeo\""); // \- to avoid "" confusion

    let filepath = r"C:\My Document\new\videos";    // r - raw string - to avoid confusion of \
    println!("{filepath}");

}   
