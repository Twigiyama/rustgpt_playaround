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

    //floating point
    let freezing_temp: f64 = -2.4;
    println!("Freezing temp is {}", freezing_temp);


    let is_zero_remainder: bool = (10 % 4) == 0;
    println!("Is zero remainder? {}", is_zero_remainder);

    let my_char: char = 'z';
    println!("My char is {}", my_char);

    let emoji_char: char = '😀';
    println!("My emoji char is {}", emoji_char);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:#?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("my_floats_new is {:#?}", my_floats_new);

}
