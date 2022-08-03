pub fn run() {

    let age: u8 = 24;
    let check_id: bool = false; 
    let knows_person_of_age = true;

    if age >= 25 && check_id  || knows_person_of_age {
        println!("Allowed to drink.");
    } else if age < 25 && check_id {
        println!("Here's your juice.");
    }

}