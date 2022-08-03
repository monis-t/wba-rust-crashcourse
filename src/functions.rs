fn greeting(greet: &str, name: &str) {
    println!("{}{}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 
}



pub fn run() {
    greeting("hello ", "wba");

    let n3 = add(1,2);
    println!("{}", n3);

    //closures
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3,2));

}
