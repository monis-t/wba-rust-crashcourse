// referance pointers refer to a resource in memory.


pub fn run() {

    // Prmitive array 
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?} {:?}", arr1, arr2);

    // with non-primitves, if you assign another variable
    // to a peice of data, the first varibale will 
    // no longer hold that value. You'll need t0
    // referance (&) to point to the resource.

    //vectors
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("{:?} {:?}", vec1, vec2);

}