pub fn run() {

    let mut count: u8 = 0;
    loop {
        count += 1;
        println!("number {}", count);

        if count == 20{
            break;
        }
    }
    //while loop

    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    //in range
    for count in 0..100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }
}