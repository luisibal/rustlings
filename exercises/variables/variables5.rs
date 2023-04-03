// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    let mut number2:i32 = 5; // don't change this line
    {
        let number = 3; // don't rename this variable
        number2 += number;
        println!("Number plus two is : {}", number + 2);
    }
    println!("Number 2 : {}", number2 + 5);
    println!("Spell a Number : {}", number);
}
