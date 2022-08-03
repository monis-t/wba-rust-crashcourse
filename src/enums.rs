//enums are types which have a few definite values called variants.

enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {

    //perform action pending
    match m {
        Movement::Up => println!("UP"),
        Movement::Down =>println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right =>println!("Right"),
    }
}

pub fn run() {

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}