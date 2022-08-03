pub fn run() {


    let mut hello = String::from("HELLO ");
    println!("{}", hello.len());

    //push char
    hello.push('w');

    //push string
    hello.push_str("orld");

    println!("{}", hello);

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // to check if string is empty
    println!("is empty : {}", hello.is_empty());

    //does it contain substring?
    println!("contains : {}", hello.contains("world"));

    //replace 
    println!("replaced : {} ", hello.replace("world", "there"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    
    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //asssertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}",s);
}