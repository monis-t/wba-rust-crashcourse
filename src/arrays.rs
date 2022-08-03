pub fn run() {

    let mut a: [i32; 4]= [1,2,3,4];
    a[2] = 31;
    println!("{:?}", a);
    println!("{:?}", a[1]);

    println!("{:?}", a.len());

    //
    println!("{:?}", std::mem::size_of_val(&a));
 
    // slicing an array
    let slice: &[i32] = &a[0..4];
    println!("{:?}", slice);
}