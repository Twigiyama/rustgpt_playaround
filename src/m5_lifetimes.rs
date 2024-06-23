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