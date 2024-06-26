
#[cfg(test)]
mod test {
    use super::*;
}

#[test]
    fn tests_match_literals() {
       let number: i32 = 20;
        let res = match number {
            1 => "This is the One",
            2 | 3 | 5 | 7 | 11 => "Primed for success",
            _ => " It was something else."
        };

        println!("The result is: {}", res);

    }