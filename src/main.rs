const OUR_COURSE: &str = "Rust with AutGPT";

fn main() {
    println!("Welcome to this course on {}", OUR_COURSE);

    //Stack
    let x: i32;
    x = 42;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    //for loop
    for i in 0..=y {
        if i != 4 {
            println!("i is {}", i);
        } else {
            println!("i is {}", i);
        }
    }

    //Mutability
    let mut z: i32 = 5;
    print!("Z was {} ", z);
    z = 10;
    println!("Z is now {}", z);
}
