pub fn run() {

    let mut a: Vec<i32> = vec![1,2,3,4];
    a[2] = 31;

    // adding values
    a.push(2);
    
    println!("{:?}", a);
    println!("{:?}", a[1]);

    //vector length
    println!("{:?}", a.len());

    //size of array in memory
    println!("{:?}", std::mem::size_of_val(&a));
 
    // slicing an array
    let slice: &[i32] = &a[0..5];
    println!("{:?}", slice);

    //loop through vector values
    for x in a.iter () {
        println!("{:?}", x);
    }

    //loop and mutate values
    for x in a.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", a);

}