pub fn run() {

    // type i32 by default.
    let x = 1;

    //type f64 due to decimal by default.
    let y = 1.1;

    //type i64 by default.
    let z = 111111111;

    //finding max size of each type.
    println!("{:?}", std::i32::MAX);
    println!("{:?}", std::i64::MAX);
    println!("{:?}", std::f64::MAX);


    //Booleans
    let is_a_rustacean: bool = true;
    println!("{:?}", is_a_rustacean);

    //get boolean from expression
    let is_greater = 10 > 3;
    println!("{:?}", is_greater);

    //character
    let a = 'a';
    println!("{}", a);

}