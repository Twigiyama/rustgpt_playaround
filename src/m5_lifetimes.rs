#[allow(dead_code, unused)]

fn example_0() {
    let y: &i32;

        let x = 76;
        y = &x;



    println!("y is {}", y);
}

#[allow(dead_code, unused)]
fn example_1() {

//Allocate space in memory

let highest_age: i32;

// Initialize the variables

let alice_age: i32 = 20;
let bob_age: i32 = 25;

// Call the function
highest_age = largest(&alice_age, &bob_age);

println!("The oldest person is {}", highest_age);

fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
    if compare_1 > compare_2 {
        *compare_1
    } else {
        *compare_2
    }
}
}
#[allow(dead_code, unused)]
fn example_2() {

//Allocate space in memory

let highest_age: &i32;
let new_value: i32;
// Initialize the variables

let alice_age: i32 = 20;

{
    let bob_age: i32 = 25;

    // Call the function
    highest_age = largest(&alice_age, &bob_age);
    new_value = *highest_age;
}




println!("The oldest person is {}", new_value);

fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
    if compare_1 > compare_2 {
        compare_1
    } else {
        compare_2
    }
}
}

#[allow(dead_code, unused)]
fn example_3_generics() {

//Allocate space in memory

let highest_age: &i32;
let new_value: i32;
// Initialize the variables

let alice_age: i32 = 20;

{
    let bob_age: i32 = 25;

    // Call the function
    highest_age = largest(&alice_age, &bob_age);
    new_value = *highest_age;
}




println!("The oldest person is {}", new_value);

fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
    if compare_1 > compare_2 {
        compare_1
    } else {
        compare_2
    }
}
}

struct Person<'a> {
    name: &'a str,
    points: &'a f32
}

#[allow(dead_code, unused)]
fn example_4_with_structs() {

//Allocate space in memory

let highest_age: &i32;
let highest_points: &f32;
let new_value: i32;
let newest_value: f32;
// Initialize the variables

let alice: Person = Person {
    name: "Alice",
    points: &20.1
};

{
    let bob: Person = Person {
        name: "Bob",
        points: &25.0
    };

    // Call the function
    let next_point:&f32;
    highest_points = largest(&alice.points, &bob.points);
    next_point = largest::<f32>(alice.points, bob.points);
    newest_value = *highest_points;
}

{
    let bob_age: i32 = 25;
    let alice_age: i32 = 20;

    // Call the function
    highest_age = largest(&alice_age, &bob_age);
    new_value = *highest_age;
}




println!("The oldest person is {}", new_value);

fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
    if compare_1 > compare_2 {
        compare_1
    } else {
        compare_2
    }
}
}