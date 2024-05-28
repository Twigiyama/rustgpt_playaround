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

    //string literals
    let my_name: &str = "John";
    println!("My name is {}", my_name);

    let dyn_name: String = String::from("Asitha");
    println!("My name is {}", dyn_name);
    println!("My name is {:p}", &dyn_name);

    let new_dyn_name = my_name.to_string();
    println!("My name is {}", new_dyn_name);

    let string_slice = &dyn_name[0..3];
    println!("My name is {}", string_slice);

    //mutable vector

    let mut characters: Vec<char> = Vec::new();
    characters.insert(0, 'h');
    characters.insert(1, 'a');
    characters.insert(2, 't');
    characters.push('s');
    println!("Characters are {:?}", characters);
    dbg!(&characters);

    let removed_char: char = characters.pop().unwrap();

    println!("Removed char is {}", removed_char);
    println!("Characters are {:?}", characters);

    characters.iter().for_each(|c| println!("Char is {}", c));

    let chars_new: Vec<char> = vec!['h', 'a', 't', 's', 'e'];

    chars_new.iter().for_each(|c| println!("Char is {}", c));

    let collected: String = chars_new.iter().collect();
    println!("Collected string is {}", collected);

    for c in chars_new {
        print!("{}", c);
        if c == 'e' {
            println!("llafield");
        }
    }

    // Closures
    let num: i32 = 5;
    let add_num: Box<dyn Fn(i32) -> i32> = Box::new(move|n: i32| n + num);
    let new_num: i32 = add_num(10);
    dbg!(new_num);

    //number literals
    println!("Big number is {}", 98_222_000);
    println!("Hex is {}", 0xff);
    println!("Octal is {}", 0o77);
    println!("Binary is {}", 0b1111_0000);
    println!("Byte is {}", b'A');

    //Raw String Literals
    let textstr: &str = r#"{This} \ is a \ "raw string literal"#;
    println!("{}", textstr);

    //Binary
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    println!("A's value is {}", a);
    println!("B's value is {}", b);

    //Logic gates
    println!("Printing in binary {:08b}", a);
    println!("Printing in binary {:08b}", b);

    println!("AND is {:08b}", a & b);
    println!("OR is {:08b}", a | b);
    println!("XOR is {:08b}", a ^ b);
    println!("NOT is {:08b}", !a);

    //bitwise operations
    println!("a << 1 {:08b}", a << 1); //shift left
    println!("a >> 1 {:08b}", a >> 1); //shift right

    // Little endian or Big endian
    let n: u16 = 0x1234;
    println!("n is {:?}", n);

    let big_endian: [u8; 2] = n.to_be_bytes();
    println!("Big endian is {:?}", big_endian);


    let little_endian: [u8; 2] = n.to_le_bytes();
    println!("Little endian is {:?}", little_endian);

}