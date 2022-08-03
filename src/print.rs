pub fn run() {
    
    // Print to console
    println!("Hello");

    // Printing undeclared variables through debug token.
    println!(" The {} is {}", "climate", "changing");

    // Positinal Arguments
    println!(" The {} is {} at a {} pace.",
    "climate", "changing", "faster");

    //Named arguments
    println!("The {temp} has gotten {degrees} degrees warmer.", 
    temp = "temperature", degrees = 1.5 );

    //placeholder traits
    println!("The climate has gotten warmer. Yes {:b} or no {:b} ?", 1,0);

    //plaeholder for debug traits
    println!("{:?}", "Hello");

    //basic maths 
    println!("10 + 10 = {}", 505 + 505 );

}