


#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Mauve,
    Cyan
}


#[derive(Debug)]
enum GivenResult <T, E> {
    Ok(T),
    Err(E)

}

#[derive(Debug)]
enum GivenOption <T> {
    None,
    Something(T)

}

fn create_car_colour_mauve() -> CarColour {
    let my_car_colour: CarColour = CarColour::Mauve;
    my_car_colour
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5{
        return GivenResult::Ok(num_check)
    } else {
        return GivenResult::Err(String::from("Number is greater than 5".to_string()))
    }
}

fn check_under_five_builtin(num_check: u8) -> Result<u8, String> {
    if num_check < 5{
        return Ok(num_check)
    } else {
        return Err(String::from("Number is greater than 5".to_string()))
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check as f32 % 10.0;
    if remainder != 0.0 {
        GivenOption::Something(remainder)
    } else {
        GivenOption::None
    }  
}

fn remainder_zero_builtin(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check as f32 % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_enums() {
    let car_colour = create_car_colour_mauve();
    dbg!(car_colour);

    let is_under_five: GivenResult<u8, String> = check_under_five(  14);
    dbg!(is_under_five);

    let remainder: GivenOption<f32> = remainder_zero(  10.0);
    dbg!(remainder);
    
    let is_under_five_builtin: Result<u8, String> = check_under_five_builtin(  4);
    dbg!(is_under_five_builtin);

    let remainder_builtin: Option<f32> = remainder_zero_builtin(  11.0);
    dbg!(remainder_builtin);
    }

    
}