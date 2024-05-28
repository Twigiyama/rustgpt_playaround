


#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Mauve,
    Cyan
}

fn create_car_colour_mauve() -> CarColour {
    let my_car_colour: CarColour = CarColour::Mauve;
    my_car_colour
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_enums() {
    let car_colour = create_car_colour_mauve();
        dbg!(car_colour);
    }
}