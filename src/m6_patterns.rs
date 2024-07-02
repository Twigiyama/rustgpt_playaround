use std::os::unix::process;

use ethers::{core::k256::sha2::digest::core_api::BlockSizeUser, utils::hex::ToHexExt};

enum Message {
    Quit,
    ChangeColour(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I quit - I've had enough.");
        },

        Message::ChangeColour(red, green, blue )=> {
            println!("Changing colour to: {}, {}, {}", red, green, blue);
        },

        Message::Move { x, y } => {
            println!("Moving to: {}, {}", x, y);
        },

        Message::Write(text) => {
            println!("Writing: {}", text);
        }
    };
}


#[cfg(test)]
mod test {
    use super::*;
}

#[test]
    fn tests_large_enum() {
        let my_quit: Message = Message::Quit;
        process_message(my_quit);

        let my_colour: Message = Message::ChangeColour((212), (134), (98));
        process_message(my_colour);

        let my_move = Message::Move { x: 10, y: 20 };
        process_message(my_move);

        let my_write = Message::Write(String::from("Bollocks"));
        process_message(my_write);
    }

#[test]
    fn tests_match_literals() {
       let number: i32 = 11;
        let res = match number {
            1 => "This is the One",
            2 | 3 | 5 | 7 | 11 => "Primed for success",
            _ => " It was something else."
        };

        println!("The result is: {}", res);

    }

    
#[test]
    fn tests_match_options() {

        let some_number: Option<i32> = Some(5);
        let prob_none: Option<i32> = None;

       let my_int = if let Some(i) = some_number {
            i
        } else {
            panic!("There was no number");
        };  

        println!("My int is: {}", my_int);

        let res: i32 = match some_number {
            Some(i ) => i,
            None => {
                panic!("There was a problem with the number")
            }
        };

        println!("Some number is {:?}", some_number);
        println!("The result is: {}", res);
    }

    #[test]
    fn tests_match_results() {

        let some_result: Result<i32, &str> = Ok(50);
        let some_eror: Result<i32, &str> = Err("There was an error");

        let res: i32 = match some_result {
            Ok(i) => i,
            Err(e) => panic!("There was an error: {}", e)
        };


        let my_int = if let Ok(i) = some_result {
            i
        } else {
            panic!("There was an error");
        };
        println!("The result is: {}", my_int);
        println!("The result is: {}", res);

    }