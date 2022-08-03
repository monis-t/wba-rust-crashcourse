pub fn run() {

    // general assignment.
    let a = 12;
    println!("{:?}", a);

    // ^ vars in rust are immutable by default.
    // To reassign value to a var use MUT keyword.


    //reassigning value to variable using mut keyword.
    let mut b =13;
    b += 1; 
    println!("{:?}", b);

   // defining constants
   const ID : i32 = 001;
   println!("ID: {}", ID );

   // assigning multiple values to vars
   let (a , b) = (1, 2);
   println!("{:?},{:?}", a ,b);

}