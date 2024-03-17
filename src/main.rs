fn main() {
    //Mutable variable
    let mut x = 1; //mutable
    println!("The value of x is: {x}");
    x = 2; //can only change value, not its type!
    println!("The value of x is: {x}");

    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    //Shadowing
    let y = 5; //immutable
    let y = y + 1; //still immutable
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}")
    }
    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len(); //data type change while using same variable name
    println!("The value of 'spaces' is: {spaces}")
}
